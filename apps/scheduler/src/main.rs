//! Task scheduler module for managing background tasks
//!
//! This module provides functionality for scheduling and managing various background tasks
//! such as cleanup operations and message retry attempts. It uses tokio-cron-scheduler
//! for task scheduling and coordinates with the service factory for task execution.

mod tasks;

use anyhow::Result;
use config::SafeConfig;
use container::ServiceFactory;
use log::info;
use tasks::{
    add_task,
    cleanup::CleanupTask,
    message_retry::{MessageSendTask, MessageType},
};
use tokio_cron_scheduler::JobScheduler;

/// Adds all scheduled tasks to the job scheduler
///
/// This function initializes and registers all background tasks that need to be
/// executed periodically, including cleanup tasks and message retry tasks.
///
/// # Arguments
///
/// * `scheduler` - A static reference to the JobScheduler instance
/// * `service_factory` - A static reference to the ServiceFactory for creating service instances
///
/// # Returns
///
/// * `Result<()>` - Success if all tasks are added successfully, or an error if any task fails to be added
async fn add_tasks(scheduler: &'static JobScheduler, service_factory: &'static ServiceFactory) -> Result<()> {
    // Add cleanup task for periodic maintenance
    add_task(scheduler, CleanupTask).await?;

    // Add retry task for failed messages
    add_task(
        scheduler,
        MessageSendTask::new(service_factory, MessageType::Failed),
    )
    .await?;

    // Add retry task for unsent messages
    add_task(
        scheduler,
        MessageSendTask::new(service_factory, MessageType::UnSent),
    )
    .await?;

    Ok(())
}

/// Main entry point for the task scheduler application
///
/// This function initializes the task scheduler, sets up the database connection,
/// registers all scheduled tasks, and manages the application lifecycle.
///
/// The function will:
/// 1. Initialize logging
/// 2. Load configuration
/// 3. Set up database connection
/// 4. Create and initialize the job scheduler
/// 5. Register all tasks
/// 6. Start the scheduler
/// 7. Wait for shutdown signal
///
/// # Returns
///
/// * `Result<()>` - Success if the application runs and shuts down properly, or an error if initialization fails
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging system
    libs::logger::init();
    info!("Starting task scheduler....");

    // Load configuration from command line arguments
    let config = SafeConfig::from_args().await?;
    let config = config.get_config().await?;

    // Initialize database connection
    let (_, database) = database::mongodb::connect(&config.database.uri, &config.database.db_name).await?;

    // Create scheduler and convert to static reference for lifetime management
    let scheduler = Box::new(JobScheduler::new().await?);
    let scheduler: &'static JobScheduler = Box::leak(scheduler);

    // Create service factory and convert to static reference
    let service_factory = Box::new(ServiceFactory::new(database.clone()));
    let service_factory: &'static ServiceFactory = Box::leak(service_factory);

    // Register all scheduled tasks
    add_tasks(scheduler, service_factory).await?;

    // Start the scheduler
    scheduler.start().await?;

    // Wait for shutdown signal (Ctrl+C)
    tokio::signal::ctrl_c().await?;
    info!("Shutting down task scheduler...");

    Ok(())
}

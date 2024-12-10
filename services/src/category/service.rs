use super::dto::*;
use super::repository::ICategoryRepository;
use crate::errors::Result;
use entities::product::Category;

pub struct CategoryService<R>
where
    R: ICategoryRepository,
{
    repo: R,
}

impl<R> CategoryService<R>
where
    R: ICategoryRepository,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create_category(&self, params: CreateCategoryParams) -> Result<()> {
        let id = libs::next_id().await;
        let (level, path) = match params.parent_id {
            Some(ref parent_id) => {
                let parent = self.repo.find_by_id(parent_id).await?.ok_or("父分类不存在")?;
                (parent.level + 1, format!("{}/{}", parent.path, id))
            }
            None => (0, id.clone()),
        };

        let category = Category::new(id, params.name, params.parent_id, level, path);

        self.repo.create(&category).await
    }

    pub async fn update_category(&self, params: UpdateCategoryParams) -> Result<()> {
        let mut category = self.repo.find_by_id(&params.id).await?.ok_or("分类不存在")?;

        params.apply(&mut category);

        self.repo.update(&category).await
    }

    pub async fn get_category_tree(&self) -> Result<Vec<CategoryDetail>> {
        let categories = self.repo.find_all().await?;
        let mut root_categories = categories
            .iter()
            .filter(|c| c.parent_id.is_none())
            .map(|c| CategoryDetail {
                id: c.base.id.clone(),
                name: c.name.clone(),
                parent_id: c.parent_id.clone(),
                level: c.level,
                path: c.path.clone(),
                description: c.description.clone(),
                image: c.image.clone(),
                sort_order: c.sort_order,
                status: c.status.clone(),
                created_at: c.base.created_at,
            })
            .collect::<Vec<_>>();

        root_categories.sort_by_key(|c| c.sort_order);
        Ok(root_categories)
    }
}

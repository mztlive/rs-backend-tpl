let project_name = variable::get("project_name");
let project_name_snake = project_name.to_snake_case();
let project_name_pascal = project_name.to_pascal_case();

variable::set("project_name_snake", project_name_snake);
variable::set("project_name_pascal", project_name_pascal); 
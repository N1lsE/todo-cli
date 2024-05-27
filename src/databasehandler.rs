pub mod database_handler {
    pub mod todo_database {
        use std::{any::Any, str::FromStr};

        use chrono::{NaiveDate, NaiveTime, Utc};

        #[derive(Debug)]
        pub enum TodoState {
            Open,
            Done,
            Deleted,
        }

        #[derive(Debug)]
        pub struct Todo {
            description: String,
            status: TodoState,
            due_date: NaiveDate,
            due_time: NaiveTime,
            done_date: Option<NaiveDate>,
            create_date: NaiveDate,
            created_by: String,
            last_changed_by: String,
            last_change_date: NaiveDate,
        }
        impl Todo {
            fn new(
                description: String,
                status: TodoState,
                due_date: NaiveDate,
                due_time: NaiveTime,
                create_date: NaiveDate,
                created_by: String,
            ) -> Todo {
                let today = Utc::now().date_naive();
                Todo {
                    description,
                    status,
                    due_date,
                    due_time,
                    done_date: None,
                    create_date,
                    created_by: created_by.clone(),
                    last_changed_by: created_by,
                    last_change_date: today,
                }
            }
        }

        pub enum DatabaseField {
            Description(String),
            Status(TodoState),
            DueDate(NaiveDate),
            DueTime(NaiveTime),
            DoneDate(Option<NaiveDate>),
            CreateDate(NaiveDate),
            CreatedBy(String),
            LastChangedBy(String),
            LastChangeDate(NaiveDate),
        }
        impl DatabaseField {
            pub fn as_str(&self) -> String {
                let db_field_str = match self {
                    DatabaseField::Description(_x) => "description",
                    DatabaseField::Status(_x) => "status",
                    DatabaseField::DueDate(_x) => "due_date",
                    DatabaseField::DueTime(_x) => "due_time",
                    DatabaseField::DoneDate(_x) => "done_date",
                    DatabaseField::CreateDate(_x) => "create_date",
                    DatabaseField::CreatedBy(_x) => "created_by",
                    DatabaseField::LastChangedBy(_x) => "last_changed_by",
                    DatabaseField::LastChangeDate(_x) => "last_change_date",
                };
                db_field_str.to_string()
            }
        }

        pub enum DeletionMethod {
            Mark,
            Delete,
        }

        /// Retrieves a `Todo` item from the database based on specified fields.
        ///
        /// # Arguments
        ///
        /// * `fields` - A vector of `DatabaseField` items used to specify which fields
        ///   to retrieve. Each field must be unique by type.
        ///
        /// # Errors
        ///
        /// Returns an error if any of the fields are specified more than once.
        /// The error message includes a list of strings, each one describing a duplication issue.
        ///
        /// # Examples
        ///
        /// ```
        /// # use todo_database::database_get_todo;
        /// # use todo_database::DatabaseField;
        /// # fn main() -> Result<(), Vec<String>> {
        /// let fields = vec![DatabaseField::Description("Hello World!"), DatabaseField::CreateDate("20.04.2024"), DatabaseField::Status("done")];
        /// let todo = database_get_todo(fields);
        /// match todo {
        ///     Ok(todo) => println!("Retrieved todo: {:?}", todo),
        ///     Err(e) => e.iter().for_each(|error| println!("Error: {}", error)),
        /// }
        /// # Ok(())
        /// # }
        /// ```
        pub fn database_get_todo(fields: Vec<DatabaseField>) -> Result<Vec<Todo>, Vec<String>> {
            let mut err: Vec<String> = Vec::new();
            let mut todos: Vec<Todo> = Vec::new();

            // check double field
            for field in &fields {
                for field2 in &fields {
                    if field.type_id() == field2.type_id() {
                        err.push(format!(
                            "you can't specify the same field twise! field: {}",
                            field.as_str()
                        ))
                    }
                }
            }
            // return if err before making request
            if err.is_empty() {
                return Err(err);
            }
            // request db
            todo!("implement database_get_todo");

            if todos.is_empty() {
                err.push("No todo with given fields was found!".to_string());
                return Err(err);
            }
            return Ok(todos);
        }

        pub fn database_inserte_todo(todo: Todo) -> Result<(), String> {
            todo!("implement database_inserte_todo");
        }
        pub fn database_change_todo(old_todo: Todo, new_todo: Todo) -> Result<(), String> {
            todo!("implement database_change_todo");
        }
        pub fn database_change_todo_Field<T: PartialEq + Clone>(
            todo: Todo,
            field: DatabaseField,
            new_value: T,
        ) -> Result<(), String> {
            todo!("implement database_change_todo_where");
        }
        pub fn database_change_todo_where<T: PartialEq + Clone>(
            field: DatabaseField,
            equ: T,
            new_value: T,
        ) -> Result<(), String> {
            todo!("implement database_change_todo_where");
        }

        pub fn database_delete_todo(todo: Todo, delMethod: DeletionMethod) -> Result<(), String> {
            match delMethod {
                DeletionMethod::Mark => todo!(),
                DeletionMethod::Delete => todo!(),
            }
            todo!("implement database_delete_todo");
        }
        pub fn database_undelete_todo(todo: Todo) -> Result<(), String> {
            todo!("implement database_undelete_todo");
        }

        pub fn database_finish_todo(todo: Todo) -> Result<(), String> {
            todo!("implement database_finish_todo");
        }
        pub fn database_unfinish_todo(todo: Todo) -> Result<(), String> {
            todo!("implement database_unfinish_todo");
        }
    }
}

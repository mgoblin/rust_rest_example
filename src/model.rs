#[crud_table]
#[derive(Clone, Debug)]
pub struct Users {
  pub id: i64,
  pub name: String,
}

#[crud_table]
#[derive(Clone, Debug)]
pub struct Users {
  pub id: u64,
  pub name: String,
}

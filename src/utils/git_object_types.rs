#[derive(clap::ValueEnum, Clone, Debug)]
pub enum GitObjectTypes {
    Blob,
    Commit,
    Tree,
    Tag,
}

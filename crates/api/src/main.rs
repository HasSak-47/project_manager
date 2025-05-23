pub struct Project{
    local_id: usize,
    name: String,
}

pub struct ExternalDatabase{
    local_id: usize,
    location: String,
}

pub struct Subproject{
    parent_id: usize,
    child_id: Vec<usize>,
}

fn main(){
}

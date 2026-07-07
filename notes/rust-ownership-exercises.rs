// FUNCIONES DE DEMOSTRACION DE OWNERSHIP,Y REFERENCIAS
// fn main() {
//     let project_name = String::from("ProjectLogr"); 
//     let project_path = String::from("/home/rustpy/work/Rust/projectlogr");
//     print_project_info(&project_name, &project_path);
//     println!("Name: {}",project_name);
//     println!("Path: {}",project_path);
// }
// fn print_project_info(name: &String, path: &String) {
//     println!("Project: {} at {}", name, path);
// }

// // FUNCION DE DEMOSTRACION REFERENCIAS MUTABLES

// fn main() {
    // let mut project_name = String::from("ProjectLogr");
    // let mut project_path = String::from("/home/rustpy/work/Rust/projectlogr");
    // 
    // update_project(&mut project_name, &mut project_path);
    // 
    // println!("Name: {}", project_name);
    // println!("Path: {}", project_path);
// }
// fn update_project(name: &mut String, path: &mut String) {
    // name.push_str(" UPDATED");
    // path.push_str(" (backup)");   
    // 
// }

// FUNCION DE DEMOSTRACION LIFETIMES
// fn main() {
//     let result = get_name();
//     println!("{}", result);
// }

// fn get_name() -> String {
//     let name = String::from("ProjectLogr");
//     name
// }


struct Project {
    name: String,
    path: String,
    language: String,
}

fn main() {
    let proj: Project = Project {
        name: String::from("ProjectLogr"),
        path: String::from("/home/rustpy/work/projectlogr"),
        language: String::from("Rust"),
    };
    let mut proj1: Project = Project {
        name: String::from("TechFusionHub"),
        path: String::from("/home/rustpy/work/techfusionhub"),
        language: String::from("Python"),
    };
    proj1.name = String::from("NuevoNombre");
    println!("Project: {}", proj.name);
    println!("Path: {}", proj.path);
    println!("language: {}",proj.language);
    println!("\n");
    //proj1
    println!("Project: {}", proj1.name);
    println!("Path: {}", proj1.path);
    println!("language: {}",proj1.language);
}

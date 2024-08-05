use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;

// Вспомогательный метод, для получения ввода от пользователя
fn get_input(query: &str) -> std::io::Result<String> {      // в метод передаем строку для вывода на экран, что требуется от пользователя
    print!("{}", query);
    std::io::stdout().flush()?;                             // очищаем входной поток

    let mut buffer = String::new();     // Буфер для сохнанения данных
    std::io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_owned())        // возвращаем строку
}

// метод с логикой
fn organize_dir(dir_path: PathBuf) {
    // Проверяем существует ли папка по этому пути
    if !dir_path.exists() {
        println!("Dir \"{}\" doesn't exists", dir_path.display());
    } else {
        // Если папка существует
        let dir_files = match dir_path.read_dir() { // Получаем все файлы из папки
            Ok(dir_files) => dir_files,
            Err(err) => {
                println!("Error opening dir \"{}\": \"{}\"", dir_path.display(), err);
                return;
            }
        };

        // проходим по всему списку файлов
        for file in dir_files {
            if let Ok(file) = file {
                // если файл папка, то пропускаем
                if file.path().is_dir() {
                    println!("Path {} is a dir, skip", file.path().display());
                    continue;
                }
                // получаем расширение файла
                let file_extension = match file.path().extension() {
                    None => {
                        println!("Can't get extension of the file: \"{}\"", file.path().display());
                        continue;
                    }
                    Some(extension) => match extension.to_str() {
                        None => continue,
                        Some(extension) => extension.to_lowercase()
                    }
                };

                // Путь к папке куда нужно переместить файл
                let extension_dir = PathBuf::from(dir_path.join(file_extension));
                // Создаем папку, если не существует
                create_dir_if_not_exists(&extension_dir);
                // перемещаем файл
                move_file(&file.path(), &extension_dir.join(file.file_name()));
            }
        }
    }
}

// Метод для создания папки если она не существует
fn create_dir_if_not_exists(dir_path: &PathBuf) {
    if !dir_path.exists() {
        if let Err(err) = fs::create_dir(dir_path) {
            println!("Error creating dir\"{}\": \"{}\"", dir_path.display(), err);
        }
    }
}

// Метод для перемещения файла
fn move_file(from: &PathBuf, to: &PathBuf) {
    if let Err(err) = fs::rename(from, to) {
        println!("Error moving file \"{}\" to \"{}\": \"{}\"", from.display(), to.display(), err);
    }
}

fn main() {
    loop {
        let dir_path = match get_input("Enter a path to the dir you want to organize: ") {
            Ok(dir_path) => dir_path,
            Err(err) => {
                println!("Error getting user input: {}", err);
                continue;
            }
        };

        let now = Instant::now();       // счетчик времени
        organize_dir(PathBuf::from(dir_path));
        println!("Time to organize: {}s", now.elapsed().as_secs_f64());
    }
}

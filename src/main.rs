mod os{
    use std::path::PathBuf;
    use std::env;
    use std::fs;
    
    
    /// Получение рабочей директории
    pub fn getcwd() -> PathBuf {
        env::current_dir().unwrap()
    }
    
    
    /// Список файлов и директорий в папке
    pub fn listdir(path: PathBuf) -> Vec<PathBuf> {
        
        let mut v = Vec::new();
        
        for path in fs::read_dir(path).unwrap() {
            v.push(path.unwrap().path());
        }
        
        return v
    }
    
    
    /// Соеденение путей PathBuf + [Путь 1, Путь 2, Путь 3]
    pub fn path_join(path: PathBuf, dir: Vec<&str>) -> PathBuf {
        let mut p = path;
        for d in dir{
            p = p.join(d)
        }
        return p
    }
    
    
    /// Получение родительской папки PathBuff
    pub fn path_parent(path: PathBuf) -> PathBuf {
        path.parent().unwrap().to_path_buf()
    }
    
    
    
    /// Имя ОС
    pub fn name() -> String{
        env::consts::OS.to_string()
    }
}

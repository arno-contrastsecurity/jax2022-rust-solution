
// fn do_something() {
//     let mut m: Option<&Movie> = None;
//     {
//         let repo: MovieRepository = MovieRepository::new();
//         m = repo.find_by_name("asdf");
//     }
//     println!("{:?}", m);
// }


#[derive(Debug)]
struct Movie {
    name: String,
    genre: Option<Genre>,
}

#[derive(Debug, Eq, PartialEq)]
enum Genre {
    Action,
    Heist,
    Romance,
}

struct MovieRepository {
    movies: Vec<Movie>,
}
impl MovieRepository {
    fn new() -> MovieRepository {
        MovieRepository {
            movies: vec![]
        }
    }

    fn find_by_name(&self, name: &str) -> Option<&Movie> {
        self.movies.iter().find(|m| &m.name == name)
    }

    fn find_by_genre(&self, genre: &Genre) -> Vec<&Movie> {
        self.movies.iter().filter(|m| if let Some(mg) = &m.genre { mg == genre } else { false } )
            .collect()
        // self.movies.iter().filter(|m| match &m.genre {
        //     None => false,
        //     Some(g) => g == genre,
        // })
        //     .collect()
    }
}
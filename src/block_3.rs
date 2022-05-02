

#[derive(Debug)]
struct Movie {
    name: String,
    genre: Option<Genre>,
    director: usize,
    actors: Vec<usize>,
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

    fn find_by_name<'a>(&'a self, name: &str, person_repo: &'a PersonRepository) -> Option<FullMovie<'a>> {
        self.movies.iter().find(|m| &m.name == name)
            .map(|m| Self::link(m, person_repo))
    }

    fn find_by_genre<'a>(&'a self, genre: &Genre, person_repo: &'a PersonRepository) -> Vec<FullMovie> {
        self.movies.iter().filter(|m| if let Some(mg) = &m.genre { mg == genre } else { false } )
            .map(|m| Self::link(m, person_repo))
            .collect()
    }

    fn link<'a>(movie: &'a Movie, person_repo: &'a PersonRepository) -> FullMovie<'a> {
        FullMovie {
            name: &movie.name,
            genre: &movie.genre,
            director: &person_repo.find_by_id(movie.director).name,
            actors: movie.actors.iter()
                .map(|p| &person_repo.find_by_id(*p).name)
                .collect()
        }
    }
}

struct FullMovie<'a> {
    name: &'a String,
    genre: &'a Option<Genre>,
    director: &'a String,
    actors: Vec<&'a String>,
}

#[derive(Debug)]
struct Person {
    id: usize,
    name: String,
}

struct PersonRepository {
    persons: Vec<Person>,
}
impl PersonRepository {
    fn find_by_id(&self, id: usize) -> &Person {
        self.persons.iter().find(|p| p.id == id).unwrap()
    }
}
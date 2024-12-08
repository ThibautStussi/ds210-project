use std::collections::{HashMap, HashSet}; //main thing
use csv::ReaderBuilder;
use serde::Deserialize;


//struct for the data (useful for impl)
//while I only plan on using school type, parental income levels, peer and self motivation, and and learning disabilities
//having this is helpful as it allows me to store of all of the data but lets me focus only on what I think is important (see above)
#[derive(Debug, Deserialize)]
struct StudentRecord {
    id: usize, //unique id for each student, not in the dataset but helpful for me
    hours_htudied: i32,
    attendance: i32,
    parental_involvement: String,
    access_to_resources: String,
    extracurricular_activities: String,
    sleep_hours: i32,
    previous_scores: i32,
    motivation_level: String,
    internet_access: String,
    tutoring_sessions: i32,
    family_income: String,
    teacher_quality: String,
    school_type: String,
    peer_influence: String,
    physical_activity: i32,
    learning_disabilities: String,
    parental_education_level: String,
    distance_from_home: String,
    gender: String,
    exam_score: i32,
}

#[derive(Debug)]
struct Graph {
    //usuze is id (number)
    //meaning I can find a specific student if needed
    //node id is the student
    //edge id is the adjacency HashSet for the student with teh given id

    nodes: HashMap<usize, StudentRecord>,
    //adjacency list (HashSet) to other students
    edges: HashMap<usize, HashSet<usize>>,
}

impl Graph {
    //this creates the graph
    fn new()-> self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    //adding a student based on their StudentRecord
    //does not do their edges
    fn add_student(&mut self, student: StudentRecord) {
        self.nodes.insert(student.id, student);
        self.edges.entry(student.id).or_insert_with(HashSet::new());
    }

    //adds an edge
    fn add_edge(&mut self, id1: usize, id2: usize) {
        if let Some(neighbors) = self.edges.get_mut(&id1) {
            neighbors.insert(id2);
        }
        if let Some(neighbors) = self.edges.get_mut(&id2) {
            neighbors.inster(id1);
        }
    }
}

//take and heavily edited from my hw9 code but struicture is the same
//instead it also takes in a new graph, which I then build off of, this might be dumb but let's try it out
fn read_csv(path: &str, graph: &mut Graph) -> Result<(), Box<dyn Error>> {
    //yes headers reader
    let mut reader = ReaderBuilder::new().has_headers(true).from_path(path)?;
    let mut id_count = 1; //id number that we use, go 1 at a time

    //over reach items in the csv
    for result in reader.deserialize() {
        //each line as a StudentRecord
        let student: StudentRecord = result?;
       
        //add each line to the graph as its own node (no edges)
        graph.add_student(id_count, student);

        //increment id counter
        id_count += 1;
    }

    let ids: Vec<usize> = graph.nodes.keys().cloned().collect();
    for i in 0..ids.len() {
        //iterates over every student-student connection
        //do i, i+1 since for connection 20, you've4 already checked 1-19 so no need to repeat
        //slow for the first half, second half a lot faster
        for j in (i + 1)..ids.len() {
            //get their ndoe
            let student1 = &graph.nodes[&ids[i]];
            let student2 = &graph.nodes[&ids[j]];

            //since only testing school type, parental income levels, peer and self motivation, and and learning disabilities
            //run an if each time
            if 
        }
    }
}


fn main() {
    println!("test");
}

/* THIBAUT STUSSI'S DS210 FINAL PROJECT */
/* GRAPH MODULE */
/* Thibaut Stussi | thibauts@bu.edu */


/* GRAPH MODULE */
//This module craetes the Graph and StudentRecord structs, loads the data, and runs the graph analysis
//Referenced in tree.rs for making the DecisionTree

use std::collections::{BinaryHeap, HashMap, HashSet}; //main thing for the structs
use serde::Deserialize;
use std::error::Error;
use rand::Rng; //given feedback from the professor, I am using this for testing
use std::cmp::Ordering; //dijkstra's algo

//the following are all for Dijkstra's algorithm, slightly modified
#[derive(Debug, Clone, PartialEq)]
struct Node {
    id: usize,
    distance: u32,
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        //min heap by distance
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


//struct for the data (useful for impl)
//while I only plan on using school type, parental income levels, peer and self motivation, and and learning disabilities
//having this is helpful as it allows me to store of all of the data but lets me focus only on what I think is important (see above)
#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)] //some lines not used, this added to stop the warning of it
pub struct StudentRecord {
    //most entries are not used (as of now) but are kept because it is easier to load the data
    pub hours_studied: i32,
    pub attendance: i32,
    pub parental_involvement: String,
    pub access_to_resources: String,
    pub extracurricular_activities: String,
    pub sleep_hours: i32,
    pub previous_scores: i32,
    pub motivation_level: String,
    pub internet_access: String,
    pub tutoring_sessions: i32,
    pub family_income: String,
    pub teacher_quality: String,
    pub school_type: String,
    pub peer_influence: String,
    pub physical_activity: i32,
    pub learning_disabilities: String,
    pub parental_education_level: String,
    pub distance_from_home: String,
    pub gender: String,
    pub exam_score: i32,
}

impl StudentRecord {
    //helps me get the attribute give a string
    //added all types for redundancy (probably not going to use them all)
    pub fn get_attribute(&self, a: &str) -> Option<String> {
        match a {
            "hours_studied" => Some(self.hours_studied.to_string()),
            "attendance" => Some(self.attendance.to_string()),
            "parental_involvement" => Some(self.parental_involvement.clone()),
            "access_to_resources" => Some(self.access_to_resources.clone()),
            "extracurricular_activities" => Some(self.extracurricular_activities.clone()),
            "sleep_hours" => Some(self.sleep_hours.to_string()),
            "previous_scores" => Some(self.previous_scores.to_string()),
            "motivation_level" => Some(self.motivation_level.clone()),
            "internet_access" => Some(self.internet_access.clone()),
            "tutoring_sessions" => Some(self.tutoring_sessions.to_string()),
            "family_income" => Some(self.family_income.clone()),
            "teacher_quality" => Some(self.teacher_quality.clone()),
            "school_type" => Some(self.school_type.clone()),
            "peer_influence" => Some(self.peer_influence.clone()),
            "physical_activity" => Some(self.physical_activity.to_string()),
            "learning_disabilities" => Some(self.learning_disabilities.clone()),
            "parental_education_level" => Some(self.parental_education_level.clone()),
            "distance_from_home" => Some(self.distance_from_home.clone()),
            "gender" => Some(self.gender.clone()),
            "exam_score" => Some(self.exam_score.to_string()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Graph {
    //usize is id (number)
    //meaning I can find a specific student if needed
    //node id is the student
    //edge id is weighted connection between two nodes, HashMap of that

    pub nodes: HashMap<usize, StudentRecord>,
    //changed to an adjacency_list of (id, (adjacent, weight))
    pub adjacency_list: HashMap<usize, Vec<(usize, u32)>>,
}

impl Graph {
    //this creates the graph
    pub fn new()-> Self {
        Graph {
            nodes: HashMap::new(),
            adjacency_list: HashMap::new(),
        }
    }

    //adding a student based on their StudentRecord
    //does not do their edges
    pub fn add_student(&mut self, student: StudentRecord, id: usize) {
        self.nodes.insert(id, student);
        self.adjacency_list.entry(id).or_insert(vec![]);
    }

    //adds an edge
    pub fn add_edge(&mut self, id1: usize, id2: usize, weight: u32) {
        //added edge case detection of same id input
        if id1 != id2 {
            self.adjacency_list.entry(id1).or_default().push((id2, weight));
            self.adjacency_list.entry(id2).or_default().push((id1, weight));
        }
    }

    //modified print function that prints out a select amount, since all would be too much
    pub fn print(&self, mut lines1: i32, mut lines2: i32) {
        println!("Graph nodes: \n");
        //prints a selection of student and ids up to the number you give in the call
        for (id, student) in &self.nodes.clone() {
            if lines1 > 0 {
                println!("Student with id {} has characteristics: {:?}", id, student);
                lines1 += -1;
            }
        }

        println!("\nGraph connections: \n");
        //same as above but for edges
        for (id1, neighbors) in &self.adjacency_list {
            if lines2 > 0 {
                for &(neighbor, weight) in neighbors {
                    println!("Student with id {} is connected to {} with a weight of {}", id1, neighbor, weight);
                }
                lines2 += -1;
            }
        }
    }

    //function for degree centrality
    pub fn degree_centrality(&self) -> HashMap<&usize, i32> {
        let mut cent: HashMap<&usize, i32> = HashMap::new();

        //iteartes over each degree
        for (id1, neighbors) in &self.adjacency_list {
            //counts the amount of connections
            let degree = neighbors.len() as i32;
            cent.insert(id1, degree);
        }
        //returns it
        cent
    }

    //finds and outputs clusters of vectors
    //takes in cluster parameters (otherwise it is just one cluster)
    //connected components but I use this term becasue its easier to understand
    pub fn clusters(&self, weight: u32, filter: Option<Vec<&str>>) -> Vec<Vec<usize>> {
        let mut visited: HashSet<usize> = HashSet::new();
        let mut parts: Vec<Vec<usize>> = Vec::new();

        for &node in self.nodes.keys() {
            //for every node make sure it has not been visited yet
            if !visited.contains(&node) {
                //stack for processing, and part being the single cluster (will become a vec in parts)
                let mut part = Vec::new();
                let mut stack = vec![node];
                
                while let Some(x) = stack.pop() {
                    if visited.insert(x) {
                        part.push(x);

                        for (neighbor, w) in &self.adjacency_list[&x] {
                            //weight threshold
                            if w >= &weight && !visited.contains(&neighbor) {
                                //checks a filter to see if attributes are the same, pushes if yes
                                if let Some(attributes) = &filter {
                                    if attributes.iter().all(|a| { 
                                        self.nodes[&x].get_attribute(a) == self.nodes[&neighbor].get_attribute(a) }) {
                                            stack.push(*neighbor);
                                        }
                                }
                                else {
                                    stack.push(*neighbor);
                                }
                            }
                        }
                    }
                }
                parts.push(part);
            }
        }
        parts 
    }

    //shortest path from id1 to any other node
    //CHANGE TO Dijkstra's
    pub fn shortest_path(&self, id1: usize) -> HashMap<usize, u32> {
        //this is now done using Dijkstra's algorithm
        let mut distances: HashMap<usize, u32> = HashMap::new();
        let mut prio_q: BinaryHeap<Node> = BinaryHeap::new();

        //initializes max distance for no connection
        for &node in self.nodes.keys() {
            distances.insert(node, u32::MAX);
        }

        //start being zero dist
        distances.insert(id1, 0);
        prio_q.push(Node { id: id1, distance: 0 });

        while let Some(Node {id, distance }) = prio_q.pop() {
            if distance > *distances.get(&id).unwrap_or(&u32::MAX) {
                continue;
            }
            //weight accounted for
            for &(neighbor, weight) in &self.adjacency_list[&id] {
                let new_dist = distance + weight;
                    if &new_dist < distances.get(&neighbor).unwrap_or(&u32::MAX) {
                        distances.insert(neighbor, new_dist);
                        prio_q.push(Node { id: neighbor, distance: new_dist })
                    }
            }
        }
        distances
    }
    
    //calcualtes closeness centrality for each point
    //does the basic reciprocal sum of shortest distances, nothing complicated
    pub fn closeness_centrality(&self) -> HashMap<usize, f64> {
        let mut closeness_cent: HashMap<usize, f64> = HashMap::new();

        for &id in self.nodes.keys() {
            let shortest_paths = self.shortest_path(id);
            //since u32::MAX is used for not connected, just remove them (prob won't change much since 1/u32::MAX is v small but still)
            let sum: u32 = shortest_paths.values().filter_map(|dist| 
                if *dist == u32::MAX { None } else { Some(*dist) }).sum();
            //div by 0 error stop
            if sum > 0 {
                let closeness = 1.0 / (sum as f64);
                closeness_cent.insert(id, closeness);
            }
            else {
                closeness_cent.insert(id, 0.0);
            }
        }
        closeness_cent
    }
}

//take and heavily edited from my hw9 code but structure is the same

/* IMPORTANT */
//ONLY *percent* of the data is used given how dense the graph it, I need to work on this more later
//this was told to me by Prof. Chator to do, shouldn't affect overall analysis tho

/* UPDATE */
//now takes in two graphs, a test and train graph
//you can determine how much of the overall data goes into the first (train) and second (test) graph with a new parameter
//IMPORTANT: percent needs to be a value between 0.0-1.0.
pub fn read_csv(path: &str, graph1: &mut Graph, graph2: &mut Graph, percent: f64) -> Result<(), Box<dyn Error>> {
    //yes headers reader
    //for some reason I do not need to import use csv::ReaderBuilder;??? eh if it works it works
    let mut reader = csv::ReaderBuilder::new().has_headers(true).from_path(path)?;
    let mut id_count = 1; //id number that we use, go 1 at a time
    let mut rng = rand::thread_rng();

    //over reach items in the csv
    for result in reader.deserialize() {
        //each line as a StudentRecord
        let student: StudentRecord = result?;
        //add each line to the graph as its own node (no edges)
        //20% chance to add since it is hard to run all these commands on such a large graph
        if rng.gen_bool(percent) {
            graph1.add_student(student, id_count);
        }
        else {
            graph2.add_student(student, id_count);
        }
        //increment id counter
        id_count += 1;
    }

    //runs it over the first graph
    let ids1: Vec<usize> = graph1.nodes.keys().cloned().collect();
    for i in 0..ids1.len() {
        //iterates over every student-student connection
        //do i, i+1 since for connection 20, you've4 already checked 1-19 so no need to repeat
        //slow for the first half, second half a lot faster
        for j in (i + 1)..ids1.len() {
            //get their ndoe
            let student1 = &graph1.nodes[&ids1[i]];
            let student2 = &graph1.nodes[&ids1[j]];
            let weight: u32 = calc_weight(student1, student2);
            //if there is a connection (weight > 0) add an edge between both ids
            if weight > 0 {
                graph1.add_edge(ids1[i], ids1[j], weight);
            }
        }
    }
    
    //runs it over the second graph too (repeat code)
    let ids2: Vec<usize> = graph2.nodes.keys().cloned().collect();
    for i in 0..ids2.len() {
        //iterates over every student-student connection
        //do i, i+1 since for connection 20, you've4 already checked 1-19 so no need to repeat
        //slow for the first half, second half a lot faster
        for j in (i + 1)..ids2.len() {
            //get their ndoe
            let student1 = &graph2.nodes[&ids2[i]];
            let student2 = &graph2.nodes[&ids2[j]];
            let weight: u32 = calc_weight(student1, student2);
            //if there is a connection (weight > 0) add an edge between both ids
            if weight > 0 {
                graph2.add_edge(ids2[i], ids2[j], weight);
            }
        }
    }
    Ok(())
}

//calculates the weight of the connection between students
//since only testing school type, parental income levels, peer and self motivation, and and learning disabilities
//probably can just be inside read_csv but keeping it as a func in case I need it ever somehow idk
pub fn calc_weight(student1: &StudentRecord, student2: &StudentRecord) -> u32 {
    let mut weight: u32 = 0;
    if student1.school_type == student2.school_type { weight += 1; }
    if student1.family_income == student2.family_income { weight += 1; }
    if student1.motivation_level == student2.motivation_level { weight += 1; }
    if student1.peer_influence == student2.peer_influence { weight += 1; }
    //only testing in this case for having a disability
    if student1.learning_disabilities == "Yes" && student2.learning_disabilities == "Yes" { weight += 1; }
    weight
}
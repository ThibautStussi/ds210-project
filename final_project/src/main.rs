use std::collections::{HashMap, HashSet, VecDeque}; //main thing for the structs
use serde::Deserialize;
use std::error::Error;
use std::collections::BinaryHeap;

//struct for the data (useful for impl)
//while I only plan on using school type, parental income levels, peer and self motivation, and and learning disabilities
//having this is helpful as it allows me to store of all of the data but lets me focus only on what I think is important (see above)
#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)] //some lines not used, this added to stop the warning of it
struct StudentRecord {
    //most entries are not used (as of now) but are kept because it is easier to load the data
    hours_studied: i32,
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

impl StudentRecord {
    //helps me get the attribute give a string
    //added all types for redundancy (probably not going to use them all)
    fn get_attribute(&self, a: &str) -> Option<String> {
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

#[derive(Debug)]
struct Graph {
    //usuze is id (number)
    //meaning I can find a specific student if needed
    //node id is the student
    //edge id is weighted connection between two nodes, HashMap of that

    nodes: HashMap<usize, StudentRecord>,
    //changed to a HashMap of a tuple of ids (usize), followed by a u32 (weight of connection)
    edges: HashMap<(usize, usize), u32>,
}

impl Graph {
    //this creates the graph
    fn new()-> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    //adding a student based on their StudentRecord
    //does not do their edges
    fn add_student(&mut self, student: StudentRecord, id: usize) {
        self.nodes.insert(id, student);
    }

    //adds an edge
    fn add_edge(&mut self, id1: usize, id2: usize, weight: u32) {
        //added edge case detection of same id input
        if id1 != id2 {
            //makes the tuple
            //use min/max to have a constant ordering of edge ids
            let edge: (usize, usize) = (id1.min(id2), id1.max(id2));

            //adds the edge (above) plus the weight (either combinding weights or just adding the weight)
            self.edges.entry(edge).and_modify(|w: &mut u32| *w += weight).or_insert(weight);
        }
    }

    //modified print function because printing out each entry and node would take ages
    fn print(&self, mut lines1: i32, mut lines2: i32) {
        println!("Graph nodes: \n");
        //prints a selection of student and ids up to the number you give in the call
        for (id, student) in &self.nodes.clone() {
            if lines1 != 0 {
                println!("Student   / id: {} has characteristics {:?}", id, student);
                lines1 += -1;
            }
        }

        println!("\nGraph connections: \n");
        //same as above but for edges
        for ((id1, id2), weight) in &self.edges.clone() {
            if lines2 != 0 {
                println!("Ids: {} and {} have a weight of: {}", id1, id2, weight);
                lines2 += -1;
            }
        }
    }

    //function for degree centrality
    fn degree_centrality(&self) -> HashMap<&usize, i32> {
        let mut cent: HashMap<&usize, i32> = HashMap::new();

        //iteartes over each degree
        for ((id1, id2), __) in &self.edges {
            //counts the amount of connections
            *cent.entry(id1).or_insert(0) += 1;
            *cent.entry(id2).or_insert(0) += 1;
        }

        //returns it
        cent
    }

    //finds and outputs clusters of vectors
    //takes in cluster parameters (otherwise it is just one cluster)
    fn clusters(&self, weight: u32, filter: Option<Vec<&str>>) -> Vec<Vec<usize>> {
        //visited nodes as to not go over it again
        let mut visited = HashSet::new();

        //creates a vector of vector to store each cluster (each is its own vector)
        let mut parts = Vec::new();

        for &node in self.nodes.keys() {
            //for every node make sure it has not been visited yet
            if !visited.contains(&node) {
                //stack for processing, and part being the single cluster (will become a vec in parts)
                let mut part = Vec::new();
                let mut stack = vec![node];

                
                while let Some(x) = stack.pop() {
                    if visited.insert(x) {
                        part.push(x);

                        for (&(id1, id2), &w) in &self.edges {
                            //checks which node of the pair is being analyzed (if somehow not, continue)
                            let neighbor = if id1 == x { id2 } else if id2 == x { id1 } else { continue };

                            //weight threshold
                            if w >= weight && !visited.contains(&neighbor) {
                                //checks a filter to see if attributes are the same, pushes if yes
                                if let Some(attributes) = &filter {
                                    if attributes.iter().all(|a| { 
                                        self.nodes[&x].get_attribute(a) == self.nodes[&neighbor].get_attribute(a) }) {
                                            stack.push(neighbor);
                                        }
                                }
                                else {
                                    stack.push(neighbor);
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
    //HashMap is <id, dist>
    //this is taking forever, probably not feasible??????
    fn shortest_path(&self, id1: usize) -> HashMap<usize, u32> {
        let mut distances = HashMap::new();
        let mut visited = HashSet::new();
        //use VecDeque for queue since its the fastest way + easy to use
        let mut q = VecDeque::new();

        //make sure the start is accounted for
        distances.insert(id1, 0);
        visited.insert(id1);
        q.push_back(id1);

         while let Some(current) = q.pop_front() {
            for (&(id1, id2), _) in &self.edges {
                if id1 == current || id2 == current {
                    let neighbor = if id1 == current { id2 } else { id1 };

                    println!("working on {}", current);
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        distances.insert(neighbor, distances[&current] + 1);
                        q.push_back(neighbor);
                    }
                }
            }
        }
        distances
    }
    
    //calcualtes closeness centrality for each point
    //does the basic reciprocal sum of shortest distances, nothing complicated
    fn closeness_centrality(&self) -> HashMap<usize, f64> {
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

//take and heavily edited from my hw9 code but struicture is the same
//instead it also takes in a new graph, which I then build off of, this might be dumb but let's try it out
fn read_csv(path: &str, graph: &mut Graph) -> Result<(), Box<dyn Error>> {
    //yes headers reader
    //for some reason I do not need to import use csv::ReaderBuilder;??? eh if it works it works
    let mut reader = csv::ReaderBuilder::new().has_headers(true).from_path(path)?;
    let mut id_count = 1; //id number that we use, go 1 at a time

    //over reach items in the csv
    for result in reader.deserialize() {
        //each line as a StudentRecord
        let student: StudentRecord = result?;
       
        //add each line to the graph as its own node (no edges)
        graph.add_student(student, id_count);

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

            let weight: u32 = calc_weight(student1, student2);

            //if there is a connection (weight > 0) add an edge between both ids
            if weight > 0 {
                graph.add_edge(ids[i], ids[j], weight);
            }
        }
    }

    Ok(())
}

//calculates the weight of the connection between students
//since only testing school type, parental income levels, peer and self motivation, and and learning disabilities
fn calc_weight(student1: &StudentRecord, student2: &StudentRecord) -> u32 {
    let mut weight: u32 = 0;

    if student1.school_type == student2.school_type {
        weight += 1;
    }
    if student1.family_income == student2.family_income {
        weight += 1;
    }
    if student1.motivation_level == student2.motivation_level {
        weight += 1;
    }
    if student1.peer_influence == student2.peer_influence {
        weight += 1;
    }
    //only testing in this case for having a disability
    if student1.learning_disabilities == "Yes" && student2.learning_disabilities == "Yes" {
        weight += 1;
    }

    weight
}

fn main() {
    let mut graph = Graph::new();

    let _read_csv = read_csv("StudentPerformanceFactors.csv", &mut graph);

    graph.print(5, 100);

    /* DEGREE CENTRALITY
    let centrality: HashMap<&usize, i32> = graph.degree_centrality();
    println!("Degree centrality:");
    println!("{:?}", centrality);
    */

    
    /* CLUSTER NODES
    let clusters = graph.clusters(3, Some(vec!["school_type", "family_income"]));
    println!("Clusters of nodes:");
    println!("{:?}", clusters);
    */

    //println!("{:?}", graph.shortest_path(10));

    /* CLOSENESS CENTRALITY 
    let close_cent = graph.closeness_centrality();
    println!("Closeness centrality:");
    println!("{:?}", close_cent);*/
}

//tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    //tests shortest path
    #[test]
    fn test_shortest_path() {
        let mut graph = Graph::new();

        let student1 = StudentRecord {
            school_type: "Public".to_string(),
            family_income: "High".to_string(),
            motivation_level: "High".to_string(),
            peer_influence: "Positive".to_string(),
            learning_disabilities: "No".to_string(),
            ..Default::default()
        };

        let student2 = StudentRecord {
            school_type: "Private".to_string(),
            family_income: "Medium".to_string(),
            motivation_level: "Medium".to_string(),
            peer_influence: "Negative".to_string(),
            learning_disabilities: "No".to_string(),
            ..Default::default()
        };

        let student3 = StudentRecord {
            school_type: "Public".to_string(),
            family_income: "Low".to_string(),
            motivation_level: "Low".to_string(),
            peer_influence: "Positive".to_string(),
            learning_disabilities: "Yes".to_string(),
            ..Default::default()
        };

        graph.add_student(student1, 1);
        graph.add_student(student2, 2);
        graph.add_student(student3, 3);

        graph.add_edge(1, 2, 1);
        graph.add_edge(2, 3, 1);
        graph.add_edge(3, 3, 2);

        let dists = graph.shortest_path(1);

        println!("{:?}", dists);

        let expected_dists: HashMap<usize, u32> = HashMap::from([(1, 0), (2, 1), (3, 2),]);

        for (id, &expected_dists) in expected_dists.iter() {
            assert_eq!(dists.get(id), Some(&expected_dists));
        }
    }
}
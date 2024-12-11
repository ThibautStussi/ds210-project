use std::collections::{BinaryHeap, HashMap, HashSet}; //main thing for the structs
use serde::Deserialize;
use std::error::Error;
use rand::Rng; //given feedback from the professor, I am using this for testing
use std::cmp::Ordering; //dijkstra's algo
//decision trees
use linfa::prelude::*;
use linfa::dataset::Dataset;
use linfa_trees::DecisionTree;
use ndarray::Array2;

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
    //changed to an adjacency_list of (id, (adjacent, weight))
    adjacency_list: HashMap<usize, Vec<(usize, u32)>>,
}

impl Graph {
    //this creates the graph
    fn new()-> Self {
        Graph {
            nodes: HashMap::new(),
            adjacency_list: HashMap::new(),
        }
    }

    //adding a student based on their StudentRecord
    //does not do their edges
    fn add_student(&mut self, student: StudentRecord, id: usize) {
        self.nodes.insert(id, student);
        self.adjacency_list.entry(id).or_insert(vec![]);
    }

    //adds an edge
    fn add_edge(&mut self, id1: usize, id2: usize, weight: u32) {
        //added edge case detection of same id input
        if id1 != id2 {
            self.adjacency_list.entry(id1).or_default().push((id2, weight));
            self.adjacency_list.entry(id2).or_default().push((id1, weight));
        }
    }

    //modified print function because printing out each entry and node would take ages
    fn print(&self, mut lines1: i32, mut lines2: i32) {
        println!("Graph nodes: \n");
        //prints a selection of student and ids up to the number you give in the call
        for (id, student) in &self.nodes.clone() {
            if lines1 > 0 {
                println!("Student   / id: {} has characteristics {:?}", id, student);
                lines1 += -1;
            }
        }

        println!("\nGraph connections: \n");
        //same as above but for edges
        for (id1, neighbors) in &self.adjacency_list {
            if lines2 > 0 {
                for &(neighbor, weight) in neighbors {
                    println!("Node {} borders {} with a weight of {}", id1, neighbor, weight);
                }
                lines2 += -1;
            }
        }
    }

    //function for degree centrality
    fn degree_centrality(&self) -> HashMap<&usize, i32> {
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
    fn clusters(&self, weight: u32, filter: Option<Vec<&str>>) -> Vec<Vec<usize>> {
        let mut visited = HashSet::new();
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
    fn shortest_path(&self, id1: usize) -> HashMap<usize, u32> {
        //this is now done using Dijkstra's algorithm
        let mut distances = HashMap::new();
        let mut prio_q = BinaryHeap::new();

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

            //weight ignored (it helps with calculation)
            for &(neighbor, _) in &self.adjacency_list[&id] {
                let new_dist = distance + 1;
                    if &new_dist < distances.get(&neighbor).unwrap_or(&u32::MAX) {
                        distances.insert(neighbor, new_dist);
                        prio_q.push(Node { id: neighbor, distance: new_dist })
                    }
            }
        }

        /* this is BFS and OLD but kept for reference if needed
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

                    //println!("working on {}", current);
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        distances.insert(neighbor, distances[&current] + 1);
                        q.push_back(neighbor);
                    }
                }
            }
        } */
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

    //creates the decision tree needede for decisiontree analysis
    //this took a while to code omg, so many minor bugs
    fn decision_tree(&self) -> Result<DecisionTree<f64, usize>, Box<dyn Error>> {
        let mut features:Vec<Vec<f64>> = Vec::new();
        let mut labels: Vec<usize> = Vec::new();

        //iterates through each StudentRecord to create their feature vector w/ exam score as label
        for (_id, student) in self.nodes.clone() {
            let mut feature_v = Vec::new();

            //encodes categoricals using one-hot encoding
            let school_type_encode: Vec<f64> = match student.school_type.as_str() {
                "Public" => vec![1.0, 0.0],
                "Private" => vec![0.0, 1.0],
                _ => vec![0.0, 0.0], };
            
            let family_inc_encode: Vec<f64> = match student.family_income.as_str() {
                "Low" => vec![1.0, 0.0, 0.0],
                "Medium" => vec![0.0, 1.0, 0.0],
                "High" => vec![0.0, 0.0, 1.0],
                _ => vec![0.0, 0.0, 0.0], };

            let peer_influ_encode: Vec<f64> = match student.peer_influence.as_str() {
                "Negative" => vec![1.0, 0.0, 0.0],
                "Neutral" => vec![0.0, 1.0, 0.0],
                "Positive" => vec![0.0, 0.0, 1.0],
                _ => vec![0.0, 0.0, 0.0], };
            
            let motiv_encode: Vec<f64> = match student.motivation_level.as_str() {
                "Low" => vec![1.0, 0.0, 0.0],
                "Medium" => vec![0.0, 1.0, 0.0],
                "High" => vec![0.0, 0.0, 1.0],
                _ => vec![0.0, 0.0, 0.0], };
            
            let learn_disabil_encode: Vec<f64> = match student.learning_disabilities.as_str() {
                "Yes" => vec![1.0, 0.0],
                "No" => vec![0.0, 1.0],
                _ => vec![0.0, 0.0], };

            //adds the encoded categorical variables
            feature_v.extend(school_type_encode);
            feature_v.extend(family_inc_encode);
            feature_v.extend(peer_influ_encode);
            feature_v.extend(motiv_encode);
            feature_v.extend(learn_disabil_encode);

            //adds the continous variables
            feature_v.push(student.hours_studied as f64);
            feature_v.push(student.attendance as f64);
            feature_v.push(student.previous_scores as f64);
            feature_v.push(student.tutoring_sessions as f64);

            features.push(feature_v);
            //makes the exam score the label
            labels.push(student.exam_score as usize);
        }
        
        let final_features: Array2<f64> = Array2::from_shape_vec((features.len(), features[0].len()), features.concat())?;
        let final_labels: Array2<usize> = Array2::from_shape_vec((labels.len(), 1), labels.clone())?;

        let dataset = Dataset::new(final_features, final_labels).with_feature_names(
            vec!["School Type", "Family Income", "Peer Influence", "Motivation", "Learning Disabilities", "Hours Studied",
            "Attendance", "Previous Scores", "Tutoring Sessions"]);
        //println!("\n\n\n\nDataset records: {:?}", dataset.records().shape());
        //println!("\n\n\n\nDataset targets: {:?}", dataset.targets().shape());

        //this is b/c of the randomness of the read_csv making stuff not the same
        let rows = dataset.targets.len();

        //this clones the target/records and fixes a shape issues
        //this bug took me hours to fix, ignore the insane amount of print statements here
        let records = dataset.records().clone();
        let targets = dataset.targets().clone().into_shape((rows,)).unwrap();
        let dataset = Dataset::new(records, targets);

        let model = DecisionTree::params().fit(&dataset)?;

        //println!("\n\n\n\nDataset records: {:?}", dataset.records().shape());
        //println!("\n\n\n\nDataset targets: {:?}", dataset.targets().shape());
        //println!("{:?}", model);

        Ok(model)
    }

}

//take and heavily edited from my hw9 code but struicture is the same
//instead it also takes in a new graph, which I then build off of, this might be dumb but let's try it out

/* IMPORTANT */
//ONLY ~20% of the data is used given how dense the graph it, I need to work on this more later
//this was told to me by Prof. Chator to do, shouldn't affect overall analysis tho
fn read_csv(path: &str, graph: &mut Graph) -> Result<(), Box<dyn Error>> {
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
        if rng.gen_bool(0.2) {
            graph.add_student(student, id_count);
        }

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
//probably can just be inside read_csv but keeping it as a func in case I need it ever somehow idk
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

    //graph.print(5, 5);

    /* DEGREE CENTRALITY */
    let centrality: HashMap<&usize, i32> = graph.degree_centrality();
    println!("Degree centrality:");
    println!("{:?}", centrality);

    
    /* CLUSTER NODES */
    let clusters = graph.clusters(3, Some(vec!["school_type", "family_income"]));
    println!("Clusters of nodes:");
    println!("{:?}", clusters);
    

    //this is done since I can't just do a numbered node since only 20% come through, it would fail 20% of the time
    for (id, _) in &graph.nodes {
        println!("The shortest path to all nodes from {} is {:?}", *id, &graph.shortest_path(*id));
        break
    }

    println!("\n\n\n\n\n\n");

    /* CLOSENESS CENTRALITY 
    let close_cent = graph.closeness_centrality();
    println!("Closeness centrality:");
    println!("{:?}", close_cent); */

    graph.decision_tree();
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

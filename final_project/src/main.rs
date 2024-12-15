/* THIBAUT STUSSI'S DS210 FINAL PROJECT */
//Thibaut Stussi | thibauts@bu.edu


//This project aims to understand the connection between various characteristics and exam scores
//This project uses the attached StudentPerformanceFactors.csv file taken from Kaggle
//split into main.rs that runs the code and two sub trees
//  - graph.rs that create the graph and runs all of the graph functions
//  - tree.rs that runs all the DecisionTree code

/* MODULE IMPORT */
mod graph;
mod tree;
#[allow(unused_imports)]
use graph::{StudentRecord, Graph}; //marked as unused but if I remove half the code doesn't work (make it make sense)

use std::collections::HashMap; //the only crate I need that its in a module


fn main() {
    /* BUILDING THE GRAPHS */
    let mut train_graph = graph::Graph::new();
    let mut test_graph = graph::Graph::new();

    //30% to train_graph, 70% to test_graph
    let _read_csv = graph::read_csv("StudentPerformanceFactors.csv", &mut train_graph, &mut test_graph, 0.3);

    println!("Printing 1 nodes and 1 edges:");
    train_graph.print(1, 1);

    println!("\n\n\n\n\n\n");

    /* DEGREE CENTRALITY */
    let centrality: HashMap<&usize, i32> = train_graph.degree_centrality();
    println!("Degree centrality of each node:");
    println!("{:?}", centrality);
    let mut temp: f64 = 0.0;
    for (_, x) in &centrality { temp += *x as f64; }
    println!("Average degree centrality is: {}\n
    There is a total of {} nodes in the graph, meaning that, on average, each node is connected to {:.2}% of all nodes",
    temp / (centrality.len() as f64), train_graph.nodes.len(), (temp / (centrality.len() as f64)) / (train_graph.nodes.len() as f64) * 100.0);

    println!("\n\n\n\n\n\n");

    /* CLUSTER NODES */
    let clusters = train_graph.clusters(3, Some(vec!["school_type", "family_income"]));
    println!("Clusters of nodes:");
    let mut counter = 1;
    for x in &clusters {
        println!("Cluster {} is:\n{:?}", counter, x);
        counter += 1;
    }
    //println!("{:?}", clusters);
    println!("There are {} clusters", clusters.len());
    
    /* 
    for (id, _) in &train_graph.nodes {
        println!("The shortest path to all nodes from {} is {:?}", *id, &train_graph.shortest_path(*id));
        break
    } */

    println!("\n\n\n\n\n\n");

    /* CLOSENESS CENTRALITY */
    let close_cent = train_graph.closeness_centrality();
    println!("Closeness centrality of each node:");
    println!("{:?}", close_cent);
    let mut temp: f64 = 0.0;
    for (_, x) in &close_cent { temp += x; }
    let avg = temp / (close_cent.len() as f64);
    //id, close_cent value, how much is it off by
    let mut max: (usize, f64, f64) = (0, 0.0, 0.0);
    for (id, val) in &close_cent {
        if (val - avg).abs() > (max.2).abs() {
            max.0 = *id;
            max.1 = *val;
            max.2 = val - avg;
        }
    }
    println!("Average degree centrality is: {}", avg);
    println!("Maximum difference from average is by node {}, with value {}, that is {} ({:.2}%) off the average of {}",
        max.0, max.1, max.2, ((max.1 - avg) / avg) * 100.0, avg);
    
    println!("\n\n\n\n\n\n");


    /* DECISION TREE */
    let model = tree::decision_tree(&train_graph).expect("Model training error");
    println!("Decision Tree Model:\n{:?}", model);

    println!("\n\n\n\n\n\n");

    /* MODEL ACCURACY AND TESTING */
    println!("Testing the model on 5 students");
    let mut counter = 0;
    let test_amount = 5;
    for (id, student) in &test_graph.nodes {
        if counter < test_amount {
            let score = tree::prediction(&model, &student);
            println!("\nThe predicted score for student {} with the following traits:{:?}
            \nis {}, while their actual score is {}", id, student, score, student.exam_score);
            println!("The predicted score is {} is off the real score ({}) by {}", score, student.exam_score, (score as i32 - student.exam_score).abs());
            counter += 1;
        }
    }

    //test student is the first one in the dataset
    let test_student: StudentRecord = StudentRecord {
        hours_studied: 23,
        attendance: 84,
        parental_involvement: ("Low").to_string(),
        access_to_resources: ("High").to_string(),
        extracurricular_activities: ("No").to_string(),
        sleep_hours: 7,
        previous_scores: 73,
        motivation_level: ("Low").to_string(),
        internet_access: ("Yes").to_string(),
        tutoring_sessions: 0,
        family_income: ("Low").to_string(),
        teacher_quality: ("Medium").to_string(),
        school_type: ("Public").to_string(),
        peer_influence: ("Positive").to_string(),
        physical_activity: 3,
        learning_disabilities: ("No").to_string(),
        parental_education_level: ("High School").to_string(),
        distance_from_home: ("Near").to_string(),
        gender: ("Male").to_string(),
        exam_score: 67, };
    let guess = tree::prediction(&model, &test_student);
    println!("The test student has a predicted score of {}, his actual score is {}", guess, test_student.exam_score);

    let accuracy = tree::accuracy(&test_graph, model.clone()) * 100.0;
    println!("The model has an accuracy of: {:.2}%", accuracy);

    let importance = tree::feature_importance(&train_graph, &model.clone());
    println!("\nFeature Importance: (as percent)\n{:?}", importance);
}


/* TESTS FOR THE PROGRAM */
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_graph_new() {
        let graph = Graph::new();
        assert_eq!(graph.nodes.len(), 0);
        assert_eq!(graph.adjacency_list.len(), 0);
    }

    //degree centrality test
    #[test]
    fn test_degree_centrality() {
        let mut graph = Graph::new();
        //same students each function
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

        graph.add_edge(1,2,1);
        graph.add_edge(2, 3, 1);

        let centrality = graph.degree_centrality();

        assert_eq!(centrality.get(&1), Some(&1));
        assert_eq!(centrality.get(&2), Some(&2));
        assert_eq!(centrality.get(&3), Some(&1));
    }

    //tests closeness centrality
    #[test]
    fn test_closeness_centrality() {
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

        graph.add_edge(1,2,1);
        graph.add_edge(2, 3, 1);

        let closeness_cent = graph.closeness_centrality();

        assert!(closeness_cent.get(&1).unwrap() < closeness_cent.get(&2).unwrap());
        assert_eq!(*closeness_cent.get(&3).unwrap(), 1.0/3.0);
        assert!(closeness_cent.get(&2).unwrap() > closeness_cent.get(&3).unwrap());
    }

    //tests shortest path to ensure it works
    #[test]
    fn test_shortest_path() {
        let mut graph = Graph::new();

        //same students each funct
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
        let expected_dists: HashMap<usize, u32> = HashMap::from([(1, 0), (2, 1), (3, 2),]);

        for (id, &expected_dists) in expected_dists.iter() {
            assert_eq!(dists.get(id), Some(&expected_dists));
        }
    }
}
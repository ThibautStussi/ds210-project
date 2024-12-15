/* THIBAUT STUSSI'S DS210 FINAL PROJECT */
/* TREE.RS MODULE */
/* Thibaut Stussi | thibauts@bu.edu */


/* TREE.RS MODULE */
//This module runs all of the code for creating the DecisionTree model that predicts a student's exam score

use crate::graph::{StudentRecord, Graph};

use std::collections::HashMap; //main thing for the structs
use std::error::Error;
//decision trees
use linfa::prelude::*;
use linfa::dataset::Dataset;
use linfa_trees::DecisionTree;
use ndarray::Array2;


pub fn decision_tree(graph: &Graph) -> Result<DecisionTree<f64, usize>, Box<dyn Error>> {
    let mut features:Vec<Vec<f64>> = Vec::new();
    let mut labels: Vec<usize> = Vec::new();

    //iterates through each StudentRecord to create their feature vector w/ exam score as label
    for (_id, student) in graph.nodes.clone() {
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

//given a student, predict their exam score
//self explanatory, just makes an array using re-used code and runs a model.predict on it
pub fn prediction(model: &DecisionTree<f64, usize>, student: &StudentRecord) -> usize {
    let mut input_features = Vec::new();

    //repeat from decision_trees()
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
    input_features.extend(school_type_encode);
    input_features.extend(family_inc_encode);
    input_features.extend(peer_influ_encode);
    input_features.extend(motiv_encode);
    input_features.extend(learn_disabil_encode);
    //adds the continous variables
    input_features.push(student.hours_studied as f64);
    input_features.push(student.attendance as f64);
    input_features.push(student.previous_scores as f64);
    input_features.push(student.tutoring_sessions as f64);

    //turns the inputs into an array
    let input_array= ndarray::Array::from_shape_vec(
        (1, input_features.len()), input_features).expect("Input array error oop");
    //prediction calculation from the model
    let prediction = model.predict(&input_array);

    //println!("Predicted score: {}, actual score: {}", prediction[0], student.exam_score);
    prediction[0]
}

pub fn accuracy(graph: &Graph, model: DecisionTree<f64, usize>) -> f64 {
    let mut off_by: f64 = 0.0;
    let mut actual: f64 = 0.0;
    for (_, student) in &graph.nodes {
        let prediction = prediction(&model, student) as f64;
        actual += student.exam_score as f64;
        let off: f64 = student.exam_score as f64 - prediction;
        //println!("Predicted score: {}, actual score: {}, off by: {}", prediction, student.exam_score, off.abs());
        off_by += off.abs();
    }
    //computes error percentage (off_by/actual) and subtracts by 1 to get accuracy
    return (1.0 - (off_by / actual)) as f64
}

//helps me better understand what is going on by how influential each section is.
pub fn feature_importance(graph: &Graph, model: &DecisionTree<f64, usize>) -> Result<HashMap<String, f64>, Box<dyn Error>> {
    let mut importance: HashMap<String, f64> = HashMap::new();

    let names: Vec<(usize, &str)> = vec![
        (0, "School Type"),
        (1, "Family Income"),
        (2, "Peer Influence"),
        (3, "Motivation"),
        (4, "Learning Disabilities"),
        (5, "Hours Studied"),
        (6, "Attendance"),
        (7, "Previous Scores"),
        (8, "Tutoring Sessions"), 
        (9, "Sleep Hours"), 
        (10, "Internet Access"), 
        (11, "Extracurricular Activities"),
        (12, "Access to Resources"),
        (13, "Parental Involvement"),
        (14, "Teacher Quality"),
        (15, "Physical Activity"),
        (16, "Parental Education Level"),
        (17, "Distance from School"),
        (18, "Gender")];
    let model_accuracy = accuracy(graph, model.clone());

    //calculates accuracy differences in existing and altered model using altered_graph()
    for (x, name) in names {
        let new_graph = altered_graph(graph, x);
        let new_model = decision_tree(&new_graph);

        let new_accuracy = accuracy(&new_graph, new_model?);
        //println!("For {}, new: {}, old: {}", name, new_accuracy, model_accuracy);

        let important = (model_accuracy - new_accuracy) * 100.0;
        importance.insert(name.to_string(), important);

    }
    Ok(importance)
}

//creates a new graph with 1 variable that is different to figure out which is most important
pub fn altered_graph(graph: &Graph, feature: usize) -> Graph {
    let mut new_graph = graph.clone();

    for student in new_graph.nodes.values_mut() {
        match feature {
            //sets features to a middle-ground type of variable (or my best approximation of it)
            0 => student.school_type = String::from("Unknown"),
            1 => student.family_income = String::from("Medium"),
            2 => student.peer_influence = String::from("Neutral"),
            3 => student.motivation_level = String::from("Low"),
            4 => student.learning_disabilities = String::from("No"),
            5 => student.hours_studied = 0,
            6 => student.attendance = 50,
            7 => student.previous_scores = 50,
            8 => student.tutoring_sessions = 0,
            9 => student.sleep_hours = 8,
            10 => student.internet_access = String::from("Yes"),
            11 => student.extracurricular_activities = String::from("No"),
            12 => student.access_to_resources = String::from("Medium"),
            13 => student.parental_involvement = String::from("Medium"),
            14 => student.teacher_quality = String::from("Medium"),
            15 => student.physical_activity = 0,
            16 => student.parental_education_level = String::from("High School"),
            17 => student.distance_from_home = String::from("Near"),
            18 => student.gender = String::from("Male"),
            _ => (),
        }
    }
    new_graph
}
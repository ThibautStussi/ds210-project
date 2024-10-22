use csv::Reader;

//calcualtes summary stats (mean, median, stan dev)
fn summary_stats(reader: &mut csv::Reader<std::fs::File>, column: &str) -> (f64, f64, f64) {
    //stores all the data
    let mut rates: Vec<f64> = Vec::new();

    //finds column index to use for the .get() later on, while keeping search by regular func
    //reads the headers too
    let headers = reader.headers().expect("Unable to read headers");
    //this one took too long to figure out
    let column_index = headers.iter().position(|h| h == column).expect("Column not found");
    
    //iterates over all piece in the column to store in the rates vector (for later calc)
    //using if let allows me to remove bad data
    for result in reader.records() {
        let record = result.expect("Error reading record");
        //does above, but unwrap("0") makes all NaN/bad data a 0
        if let Ok(rate) = record.get(column_index).unwrap_or("0").parse::<f64>() {
            //using above parse, coverts data to f64 (better for symmetry)
            //pushes the value to the rates vector
            rates.push(rate);
        }
    }

    //calculates mean, median, stan dev, outputs using basic formulas/funs
    //mean is easy so done here, all other have a function b/c more complicated
    let mean = rates.iter().sum::<f64>() / rates.len() as f64;
    let median = median(&mut rates);
    //input mean to make the func easier
    let std_dev = std_deviation(&rates, mean);
    
    //returns all three as a tuple, prob improve this for later
    (mean, median, std_dev)
}

//calculates median when given a vector of values
fn median(values: &mut Vec<f64>) -> f64 {
    //sorts the data high to low to find median
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    //finds middle index
    let mid = values.len() / 2;

    //if mid = even, return average (since median will be avg)
    //if mid = odd, return the index
    if values.len() % 2 == 0 {
        (values[mid - 1] + values[mid]) / 2.0
    } 
    else {
        values[mid]
    }
}

//computes standard deviation via variance
fn std_deviation(values: &Vec<f64>, mean: f64) -> f64 {
    //calculates variance using formula (this took forever)
    let variance: f64 = values.iter().map(|value| {
        let diff = mean - (*value as f64); //difference from mean
        diff * diff //squares it for the sum
    }).sum::<f64>() / values.len() as f64; //then sums it, divides it by len

    variance.sqrt() //returns the sqrt of var for standard deviation
}

fn main() {

    //###
    //THE FOLLOWING IS FOR DATASET 1
    //###

    //creates a path to the datafiles I am using
    let iou_path = "iou_zipcodes_2020.csv";
    let non_iou_path = "non_iou_zipcodes_2020.csv";

    //loads the data using Reader object, using the path given above w/ exception
    let mut iou_reader = Reader::from_path(iou_path).expect("Unable to read the  IOU file");
    let mut non_iou_reader = Reader::from_path(non_iou_path).expect("Unable to read the non-IOU file");

    //##IMPORTANT NOTE::
    //encountering an error doing multiple prints in a row so I am just doing 1 print per compile for now (multiple compiles)


    println!("The following is summary data of the datasets for mean, median, and standard deviation:");

    //calclates the basic mean, median, standard deviation
    //println!("IOU Commercial Rate Stats: {:?}", summary_stats(&mut iou_reader, "comm_rate"));
    //println!("IOU Industrial Rate Stats: {:?}", summary_stats(&mut iou_reader, "ind_rate"));
    println!("IOU Residential Rate Stats: {:?}", summary_stats(&mut iou_reader, "res_rate"));

    //do the same but for non-IOU rates
    //println!("non-IOU Commercial Rate Stats: {:?}", summary_stats(&mut non_iou_reader, "comm_rate"));
    //println!("non-IOU Industrial Rate Stats: {:?}", summary_stats(&mut non_iou_reader, "ind_rate"));
    println!("non-IOU Residential Rate Stats: {:?}", summary_stats(&mut non_iou_reader, "res_rate"));

    //i tried to do count number of occurances of a certain string type in Rust, it was too hard
    //easier to do this in excel + using the Kaggle summary (for dataset 2)
    

    //###
    //THE FOLLOWING IS FOR DATASET 2
    //###

    let student_path = "StudentPerformanceFactors.csv";

    let mut student_reader = Reader::from_path(student_path).expect("Unable to read the student file");


    //SAME ERROR AS BEFORE, CAN ONLY OUTPUT 1 ANSWER AT A TIME
    println!("\nThe following is summary data for the student test performance dataset:");
    //println!("Hours slept stat: {:?}", summary_stats(&mut student_reader, "Sleep_Hours"));
    //println!("Previous exam score stats: {:?}", summary_stats(&mut student_reader, "Previous_Scores"));
    //println!("Tutoring sessions attended stats: {:?}", summary_stats(&mut student_reader, "Tutoring_Sessions"));
    println!("Exam score stats: {:?}", summary_stats(&mut student_reader, "Exam_Score"));

}
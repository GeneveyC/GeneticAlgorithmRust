pub struct Pvc {
    pub cities : Vec<String>,
    pub distance : Vec<Vec<u32>>,
}

impl Default for Pvc {
    fn default() -> Self {
        let cities: Vec<String> = vec!["Paris".to_string(), "Lyon".to_string(), "Marseille".to_string(), "Nantes".to_string(), "Bordeaux".to_string(), "Toulouse".to_string(), "Lille".to_string()];

        let distance_paris = vec![0, 462, 772, 379, 546, 678, 215];
        let distance_lyon = vec![462, 0, 326, 598, 842, 506, 664];
        let distance_marseille = vec![772, 326, 0, 909, 555, 407, 1005];
        let distance_nantes = vec![379, 598, 909, 0, 338, 540, 584];
        let distance_bordeaux = vec![546, 842, 555, 338, 0, 250, 792];
        let distance_toulouse = vec![678, 506, 407, 540, 250, 0, 926];
        let distance_lille = vec![215, 664, 1005, 584, 792, 926, 0];
        
        let distance = vec![distance_paris, distance_lyon, distance_marseille, distance_nantes, distance_bordeaux, distance_toulouse, distance_lille];
        Pvc { cities, distance }
    }
}

fn main() {
    println!("Hello, world!");

    let pvc_env = Pvc::default();
    println!("Distance between Paris and Lyon: {:?}", pvc_env.distance[0][1]);
}

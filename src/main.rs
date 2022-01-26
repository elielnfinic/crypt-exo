fn calculer_perimetre(longueur : i32,largeur : i32) -> i32{
    (longueur + largeur) * 2
}

fn main() {
    println!("P={}",calculer_perimetre(23,43));
}

#[cfg(test)]
mod tests{
    #[test]
    fn test_perimetre_valide(){
        assert_eq!(super::calculer_perimetre(0,0),0);        
    }

    #[test]
    fn test_perimetre_valide_2(){
        assert_eq!(super::calculer_perimetre(1,2),6);
    }

    #[test]
    fn test_perimetre_valide_3(){
        assert_eq!(super::calculer_perimetre(102000,-24),203952);
    }
}
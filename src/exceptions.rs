use std::collections::HashMap;

pub struct ExceptionMap {
    exceptions: HashMap<(String, String), String>,
}
impl ExceptionMap {
    pub fn new() -> Self {
        let mut exceptions: HashMap<(String, String), String> = HashMap::new();
        exceptions.insert(
            ("Relaxaurus".to_string(), "Sparkit".to_string()),
            "Relaxaurus Lux".to_string(),
        );
        exceptions.insert(
            ("Incineram".to_string(), "Maraith".to_string()),
            "Incineram Noct".to_string(),
        );
        exceptions.insert(
            ("Mau".to_string(), "Pengullet".to_string()),
            "Mau Cryst".to_string(),
        );
        exceptions.insert(
            ("Vanwyrm".to_string(), "Foxcicle".to_string()),
            "Vanwyrm Cryst".to_string(),
        );
        exceptions.insert(
            ("Eikthyrdeer".to_string(), "Hangyu".to_string()),
            "Eikthyrdeer Terra".to_string(),
        );
        exceptions.insert(
            ("Ephidran".to_string(), "Surfent".to_string()),
            "Elphidran Aqua".to_string(),
        );
        exceptions.insert(
            ("Pyrin".to_string(), "Katress".to_string()),
            "Pyrin Noct".to_string(),
        );
        exceptions.insert(
            ("Mammorest".to_string(), "Wumpo".to_string()),
            "Mammorest Cryst".to_string(),
        );
        exceptions.insert(
            ("Mossanda".to_string(), "Grizzbolt".to_string()),
            "Mossanda Lux".to_string(),
        );
        exceptions.insert(
            ("Dinossum".to_string(), "Rayhound".to_string()),
            "Dinossom Lux".to_string(),
        );
        exceptions.insert(
            ("Jolthog".to_string(), "Pengullet".to_string()),
            "Jolthog Cryst".to_string(),
        );
        exceptions.insert(
            ("Frostallion".to_string(), "Helzephyr".to_string()),
            "Frostallion Noct".to_string(),
        );
        exceptions.insert(
            ("Kingpaca".to_string(), "Reindrix".to_string()),
            "Ice Kingpaca".to_string(),
        );
        exceptions.insert(
            ("Lyleen".to_string(), "Menasting".to_string()),
            "Lyleen Noct".to_string(),
        );
        exceptions.insert(
            ("Leezpunk".to_string(), "Flambelle".to_string()),
            "Leezpunk Ignis".to_string(),
        );
        exceptions.insert(
            ("Blazehowl".to_string(), "Felbat".to_string()),
            "Blazehowl Noct".to_string(),
        );
        exceptions.insert(
            ("Robinquill".to_string(), "Fuddler".to_string()),
            "Robinquill Terra".to_string(),
        );
        exceptions.insert(
            ("Broncherry".to_string(), "Fuack".to_string()),
            "Broncherry Aqua".to_string(),
        );
        exceptions.insert(
            ("Surfent".to_string(), "Dumud".to_string()),
            "Surfent Terra".to_string(),
        );
        exceptions.insert(
            ("Gobfin".to_string(), "Rooby".to_string()),
            "Gobfin Ignus".to_string(),
        );
        exceptions.insert(
            ("Suzaku".to_string(), "Jormuntide".to_string()),
            "Suzaku Aqua".to_string(),
        );
        exceptions.insert(
            ("Reptyro".to_string(), "Foxcicle".to_string()),
            "Ice Reptyro".to_string(),
        );
        exceptions.insert(
            ("Hangyu".to_string(), "Swee".to_string()),
            "Hangyu Cryst".to_string(),
        );
        exceptions.insert(
            ("Mossanda".to_string(), "Petallia".to_string()),
            "Lyleen".to_string(),
        );
        exceptions.insert(
            ("Vanwyrm".to_string(), "Anubis".to_string()),
            "Faleris".to_string(),
        );
        exceptions.insert(
            ("Mossanda".to_string(), "Rayhound".to_string()),
            "Grizzbolt".to_string(),
        );
        exceptions.insert(
            ("Grizzbolt".to_string(), "Relaxaurus".to_string()),
            "Oserk".to_string(),
        );
        exceptions.insert(
            ("Kitsun".to_string(), "Astegon".to_string()),
            "Shadowbeak".to_string(),
        );
        exceptions.insert(
            ("Frostallion".to_string(), "Frostallion".to_string()),
            "Frostallion".to_string(),
        );
        exceptions.insert(
            ("Jetragon".to_string(), "Jetragon".to_string()),
            "Jetragon".to_string(),
        );
        exceptions.insert(
            ("Paladius".to_string(), "Paladius".to_string()),
            "Paladius".to_string(),
        );
        exceptions.insert(
            ("Necromus".to_string(), "Necromus".to_string()),
            "Necromus".to_string(),
        );
        exceptions.insert(
            (
                "Jormuntide Ignis".to_string(),
                "Jormuntide Ignis".to_string(),
            ),
            "Jormuntide Ignis".to_string(),
        );
        Self { exceptions }
    }

    pub fn parents_exist(&self, parents: (&str, &str)) -> Option<&str> {
        let (father, mother) = parents;
        if let Some(child) = self
            .exceptions
            .get(&(father.to_string(), mother.to_string()))
        {
            Some(child)
        } else {
            None
        }
    }
}

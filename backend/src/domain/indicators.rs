use crate::domain::stock::{Stock};

// Calcul du pourcentage de variation
pub fn calculate_change_percent(stock: &mut Stock) {
    for i in 1..stock.history.len() {
        let prev = stock.history[i - 1].price;
        let curr = stock.history[i].price;
        let change = ((curr - prev) / prev) * 100.0;
        stock.history[i].change_percent = Some(change);
    }
}

// Moyenne mobile simple sur n jours
pub fn moving_average(stock: &mut Stock, n: usize) {
    for i in 0..stock.history.len() {
        if i + 1 >= n {
            let sum: f64 = stock.history[i + 1 - n..=i]
                .iter()
                .map(|e| e.price)
                .sum();
            let ma = sum / n as f64;
            match n {
                5 => stock.history[i].moving_average_5 = Some(ma),
                10 => stock.history[i].moving_average_10 = Some(ma),
                _ => {}
            }
        }
    }
}

// Ajouter un indicateur personnalisé à toutes les entrées
pub fn add_custom_metric(stock: &mut Stock, name: &str, values: Vec<f64>) {
    for (i, val) in values.iter().enumerate() {
        if let Some(entry) = stock.history.get_mut(i) {
            entry.add_extra_metric(name, *val);
        }
    }
}

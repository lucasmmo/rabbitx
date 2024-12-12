use std::{collections::HashMap, fs, path::PathBuf, sync::mpsc, thread};

use regex::Regex;

fn main() {
    let mut regex_map = HashMap::new();
    regex_map.insert("Cloudinary", "cloudinary://.*");
    regex_map.insert("Firebase URL", ".*firebaseio.com");
    regex_map.insert(
        "Slack Token",
        "(xox[p|b|o|a]-[0-9]{12}-[0-9]{12}-[0-9]{12}-[a-z0-9]{32})",
    );
    regex_map.insert("RSA private key", "-----BEGIN RSA PRIVATE KEY-----");
    regex_map.insert("SSH (DSA) private key", "-----BEGIN DSA PRIVATE KEY-----");
    regex_map.insert("SSH (EC) private key", "-----BEGIN EC PRIVATE KEY-----");
    regex_map.insert(
        "PGP private key block",
        "-----BEGIN PGP PRIVATE KEY BLOCK-----",
    );
    regex_map.insert("Amazon AWS Access Key ID", "AKIA[0-9A-Z]{16}");
    regex_map.insert(
        "Amazon MWS Auth Token",
        "amzn\\.mws\\.[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}",
    );
    regex_map.insert("AWS API Key", "AKIA[0-9A-Z]{16}");
    regex_map.insert("Facebook Access Token", "EAACEdEose0cBA[0-9A-Za-z]+");
    regex_map.insert(
        "Facebook OAuth",
        "[f|F][a|A][c|C][e|E][b|B][o|O][o|O][k|K].*['|\"][0-9a-f]{32}['|\"]",
    );
    regex_map.insert(
        "GitHub",
        "[g|G][i|I][t|T][h|H][u|U][b|B].*['|\"][0-9a-zA-Z]{35,40}['|\"]",
    );
    regex_map.insert(
        "Generic API Key",
        "[a|A][p|P][i|I][_]?[k|K][e|E][y|Y].*['|\"][0-9a-zA-Z]{32,45}['|\"]",
    );
    regex_map.insert(
        "Generic Secret",
        "[s|S][e|E][c|C][r|R][e|E][t|T].*['|\"][0-9a-zA-Z]{32,45}['|\"]",
    );
    regex_map.insert("Google API Key", "AIza[0-9A-Za-z\\-_]{35}");
    regex_map.insert("Google Cloud Platform API Key", "AIza[0-9A-Za-z\\-_]{35}");
    regex_map.insert(
        "Google Cloud Platform OAuth",
        "[0-9]+-[0-9A-Za-z_]{32}\\.apps\\.googleusercontent\\.com",
    );
    regex_map.insert("Google Drive API Key", "AIza[0-9A-Za-z\\-_]{35}");
    regex_map.insert(
        "Google Drive OAuth",
        "[0-9]+-[0-9A-Za-z_]{32}\\.apps\\.googleusercontent\\.com",
    );
    regex_map.insert(
        "Google (GCP) Service-account",
        "\"type\", \"service_account\"",
    );
    regex_map.insert("Google Gmail API Key", "AIza[0-9A-Za-z\\-_]{35}");
    regex_map.insert(
        "Google Gmail OAuth",
        "[0-9]+-[0-9A-Za-z_]{32}\\.apps\\.googleusercontent\\.com",
    );
    regex_map.insert("Google OAuth Access Token", "ya29\\.[0-9A-Za-z\\-_]+");
    regex_map.insert("Google YouTube API Key", "AIza[0-9A-Za-z\\-_]{35}");
    regex_map.insert(
        "Google YouTube OAuth",
        "[0-9]+-[0-9A-Za-z_]{32}\\.apps\\.googleusercontent\\.com",
    );
    regex_map.insert("Heroku API Key", "[h|H][e|E][r|R][o|O][k|K][u|U].*[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}");
    regex_map.insert("MailChimp API Key", "[0-9a-f]{32}-us[0-9]{1,2}");
    regex_map.insert("Mailgun API Key", "key-[0-9a-zA-Z]{32}");
    regex_map.insert(
        "Password in URL",
        "[a-zA-Z]{3,10}://[^/\\s:@]{3,20}:[^/\\s:@]{3,20}@.{1,100}[\"'\\s]",
    );
    regex_map.insert(
        "PayPal Braintree Access Token",
        "access_token\\$production\\$[0-9a-z]{16}\\$[0-9a-f]{32}",
    );
    regex_map.insert("Picatic API Key", "sk_live_[0-9a-z]{32}");
    regex_map.insert(
        "Slack Webhook",
        "https://hooks.slack.com/services/T[a-zA-Z0-9_]{8}/B[a-zA-Z0-9_]{8}/[a-zA-Z0-9_]{24}",
    );
    regex_map.insert("Stripe API Key", "sk_live_[0-9a-zA-Z]{24}");
    regex_map.insert("Stripe Restricted API Key", "rk_live_[0-9a-zA-Z]{24}");
    regex_map.insert("Square Access Token", "sq0atp-[0-9A-Za-z\\-_]{22}");
    regex_map.insert("Square OAuth Secret", "sq0csp-[0-9A-Za-z\\-_]{43}");
    regex_map.insert("Twilio API Key", "SK[0-9a-fA-F]{32}");
    regex_map.insert(
        "Twitter Access Token",
        "[t|T][w|W][i|I][t|T][t|T][e|E][r|R].*[1-9][0-9]+-[0-9a-zA-Z]{40}",
    );
    regex_map.insert(
        "Twitter OAuth",
        "[t|T][w|W][i|I][t|T][t|T][e|E][r|R].*['|\"][0-9a-zA-Z]{35,44}['|\"]",
    );
    regex_map.insert(
        "Discord Webhook",
        r"(?:https?://)?(?:ptb.|canary.)?(?:discordapp|discord).com/api/webhooks/\d{15,20}/\S*",
    );

    let mut stack = vec![PathBuf::from(".")];
    let mut paths = Vec::new();

    let (tx, rx) = mpsc::channel::<(Regex, String, &str, PathBuf)>();

    thread::spawn(move || {
        for msg in rx {
            if let Some(matched) = msg.0.find(msg.1.as_str()) {
                println!(
                    "{} {} encontrada no arquivo: {:?}",
                    msg.2,
                    matched.as_str(),
                    msg.3.to_str().unwrap()
                )
            }
        }
    });

    while let Some(path) = stack.pop() {
        if let Ok(entries) = fs::read_dir(&path) {
            for entry in entries {
                let entry = entry.unwrap();
                let path = entry.path();
                paths.push(path.clone());
                if path.is_dir() {
                    stack.push(path);
                } else {
                    if let Ok(content) = fs::read_to_string(&path) {
                        for (k, v) in &regex_map {
                            let content = content.clone();
                            let path = entry.path().clone();
                            let re = Regex::new(&v).unwrap();
                            tx.send((re, content, k, path)).unwrap();
                        }
                    }
                }
            }
        }
    }
}

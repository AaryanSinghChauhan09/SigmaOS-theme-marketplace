// SigmaOS-theme-marketplace
// Theme discovery and download marketplace library.


/// Initialise the theme marketplace.
///
/// Call this once at application startup to set up the catalog index
/// and prepare the download subsystem.
pub fn init() {
    // TODO: load catalog index and configure download endpoints
}

pub mod icon;
pub mod palette;

// ─── Marketplace domain model ─────────────────────────────────────────

/// Marketplace listing
#[derive(Debug, Clone)]
pub struct MarketplaceListing {
    pub id: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: String,
    pub downloads: u64,
    pub rating: f32,
    pub tags: Vec<String>,
    pub is_dark: bool,
    pub accent_hex: String,
    pub installed: bool,
    pub price: f32, // 0.0 = free
}

impl MarketplaceListing {
    /// Create new listing
    pub fn new(id: &str, name: &str, author: &str) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: String::new(),
            author: author.into(),
            version: "1.0.0".into(),
            downloads: 0,
            rating: 0.0,
            tags: vec![],
            is_dark: true,
            accent_hex: String::new(),
            installed: false,
            price: 0.0,
        }
    }

    /// Set metadata
    pub fn set_metadata(&mut self, desc: &str, version: &str) {
        self.description = desc.into();
        self.version = version.into();
    }

    /// Add tag
    pub fn add_tag(&mut self, tag: &str) {
        self.tags.push(tag.into());
    }

    /// Set stats
    pub fn set_stats(&mut self, downloads: u64, rating: f32) {
        self.downloads = downloads;
        self.rating = rating.clamp(0.0, 5.0);
    }

    /// Check if free
    pub fn is_free(&self) -> bool { self.price == 0.0 }

    /// Check if matches query
    pub fn matches(&self, query: &str) -> bool {
        let q = query.to_lowercase();
        self.name.to_lowercase().contains(&q) ||
        self.description.to_lowercase().contains(&q) ||
        self.author.to_lowercase().contains(&q) ||
        self.tags.iter().any(|t| t.to_lowercase().contains(&q))
    }
}

/// Marketplace
#[derive(Debug, Default)]
pub struct ThemeMarketplace {
    listings: Vec<MarketplaceListing>,
    installed: Vec<String>, // listing IDs
}

impl ThemeMarketplace {
    pub fn new() -> Self { Self::default() }

    /// Add listing
    pub fn add_listing(&mut self, listing: MarketplaceListing) -> bool {
        if self.listings.iter().any(|l| l.id == listing.id) { return false; }
        self.listings.push(listing);
        true
    }

    /// Remove listing
    pub fn remove_listing(&mut self, id: &str) -> bool {
        let before = self.listings.len();
        self.listings.retain(|l| l.id != id);
        self.installed.retain(|i| i != id);
        self.listings.len() < before
    }

    /// Install theme
    pub fn install(&mut self, id: &str) -> bool {
        if let Some(l) = self.listings.iter_mut().find(|l| l.id == id) {
            if !l.installed {
                l.installed = true;
                self.installed.push(id.into());
                return true;
            }
        }
        false
    }

    /// Uninstall theme
    pub fn uninstall(&mut self, id: &str) -> bool {
        if let Some(l) = self.listings.iter_mut().find(|l| l.id == id) {
            if l.installed {
                l.installed = false;
                self.installed.retain(|i| i != id);
                return true;
            }
        }
        false
    }

    /// Get listing
    pub fn get_listing(&self, id: &str) -> Option<&MarketplaceListing> {
        self.listings.iter().find(|l| l.id == id)
    }

    /// List all
    pub fn list(&self) -> &[MarketplaceListing] { &self.listings }

    /// Installed listings
    pub fn installed(&self) -> Vec<&MarketplaceListing> {
        self.listings.iter().filter(|l| l.installed).collect()
    }

    /// Search
    pub fn search(&self, query: &str) -> Vec<&MarketplaceListing> {
        self.listings.iter().filter(|l| l.matches(query)).collect()
    }

    /// Free themes
    pub fn free_themes(&self) -> Vec<&MarketplaceListing> {
        self.listings.iter().filter(|l| l.is_free()).collect()
    }

    /// Top rated
    pub fn top_rated(&self, limit: usize) -> Vec<&MarketplaceListing> {
        let mut sorted: Vec<_> = self.listings.iter().collect();
        sorted.sort_by(|a, b| b.rating.partial_cmp(&a.rating).unwrap_or(std::cmp::Ordering::Equal));
        sorted.into_iter().take(limit).collect()
    }

    /// Most downloaded
    pub fn most_downloaded(&self, limit: usize) -> Vec<&MarketplaceListing> {
        let mut sorted: Vec<_> = self.listings.iter().collect();
        sorted.sort_by(|a, b| b.downloads.cmp(&a.downloads));
        sorted.into_iter().take(limit).collect()
    }

    /// Total count
    pub fn count(&self) -> usize { self.listings.len() }

    /// Installed count
    pub fn installed_count(&self) -> usize { self.installed.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn listing(id: &str, name: &str) -> MarketplaceListing {
        MarketplaceListing::new(id, name, "author")
    }

    #[test]
    fn test_listing_creation() {
        let l = MarketplaceListing::new("t1", "Nord", "dev");
        assert_eq!(l.id, "t1");
        assert_eq!(l.author, "dev");
        assert_eq!(l.version, "1.0.0");
        assert!(l.is_free());
        assert!(!l.installed);
    }

    #[test]
    fn test_listing_metadata() {
        let mut l = listing("a", "Theme");
        l.set_metadata("A nice theme", "2.0");
        l.add_tag("dark");
        l.set_stats(1000, 4.5);
        
        assert_eq!(l.description, "A nice theme");
        assert_eq!(l.version, "2.0");
        assert_eq!(l.tags.len(), 1);
        assert_eq!(l.downloads, 1000);
        assert_eq!(l.rating, 4.5);
    }

    #[test]
    fn test_listing_matches() {
        let mut l = listing("a", "Dracula");
        l.set_metadata("Vampire theme", "1.0");
        l.add_tag("purple");
        
        assert!(l.matches("dracula"));
        assert!(l.matches("vampire"));
        assert!(l.matches("purple"));
        assert!(!l.matches("nord"));
    }

    #[test]
    fn test_marketplace_add_remove() {
        let mut mp = ThemeMarketplace::new();
        assert!(mp.add_listing(listing("a", "A")));
        assert!(!mp.add_listing(listing("a", "A2"))); // dup
        assert!(mp.remove_listing("a"));
        assert_eq!(mp.count(), 0);
    }

    #[test]
    fn test_install_uninstall() {
        let mut mp = ThemeMarketplace::new();
        mp.add_listing(listing("a", "A"));
        
        assert!(mp.install("a"));
        assert!(!mp.install("a")); // already installed
        assert_eq!(mp.installed_count(), 1);
        assert_eq!(mp.installed().len(), 1);
        
        assert!(mp.uninstall("a"));
        assert_eq!(mp.installed_count(), 0);
    }

    #[test]
    fn test_search() {
        let mut mp = ThemeMarketplace::new();
        mp.add_listing(listing("nord", "Nord"));
        mp.add_listing(listing("dracula", "Dracula"));
        
        let results = mp.search("nord");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Nord");
    }

    #[test]
    fn test_free_themes() {
        let mut mp = ThemeMarketplace::new();
        let mut paid = listing("p", "Paid");
        paid.price = 4.99;
        mp.add_listing(paid);
        mp.add_listing(listing("f", "Free"));
        
        assert_eq!(mp.free_themes().len(), 1);
        assert_eq!(mp.free_themes()[0].name, "Free");
    }

    #[test]
    fn test_top_rated() {
        let mut mp = ThemeMarketplace::new();
        let mut a = listing("a", "A"); a.set_stats(100, 3.0);
        let mut b = listing("b", "B"); b.set_stats(200, 5.0);
        let mut c = listing("c", "C"); c.set_stats(300, 4.0);
        mp.add_listing(a); mp.add_listing(b); mp.add_listing(c);
        
        let top = mp.top_rated(2);
        assert_eq!(top[0].name, "B"); // 5.0
        assert_eq!(top[1].name, "C"); // 4.0
    }

    #[test]
    fn test_most_downloaded() {
        let mut mp = ThemeMarketplace::new();
        let mut a = listing("a", "A"); a.set_stats(100, 3.0);
        let mut b = listing("b", "B"); b.set_stats(500, 4.0);
        mp.add_listing(a); mp.add_listing(b);
        
        let top = mp.most_downloaded(1);
        assert_eq!(top[0].name, "B");
    }
}

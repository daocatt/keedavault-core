import Foundation

class VaultViewModel: ObservableObject {
    @Published var isVaultOpen = false
    @Published var entries: [Entry] = []
    
    private var vault: Vault?
    
    func setupDemoVault() {
        // Find Documents directory
        let docsDir = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)[0]
        let path = docsDir.appendingPathComponent("demo.kdbx").path
        
        print("Vault path: \(path)")
        
        // Use minimal params for fast demo creation
        // Note: In production, use higher iterations/memory!
        let config = VaultConfig(
            kdfIterations: 2,
            argon2Memory: 1024,      // 1MB
            argon2Parallelism: 1
        )
        
        do {
            if FileManager.default.fileExists(atPath: path) {
                print("Opening existing vault...")
                self.vault = try openVault(path: path, password: "demo")
            } else {
                print("Creating new vault...")
                self.vault = try createVault(path: path, password: "demo", config: config)
            }
            
            self.isVaultOpen = true
            loadEntries()
            print("Vault ready!")
        } catch {
            print("Error initializing vault: \(error)")
        }
    }
    
    func loadEntries() {
        guard let vault = vault else { return }
        do {
            self.entries = try vault.getEntries()
            print("Loaded \(entries.count) entries")
        } catch {
            print("Failed to load entries: \(error)")
        }
    }
    
    func addEntry(password: String) {
        guard let vault = vault else { return }
        
        // Create a new entry
        // Note: Timestamps are i64 (Unix timestamp)
        let now = Int64(Date().timeIntervalSince1970)
        
        let entry = Entry(
            id: UUID().uuidString,
            groupId: "root", // Assuming simple structure, put everything in root
            title: "Demo Entry \(entries.count + 1)",
            username: "demo_user",
            password: password,
            url: "https://example.com",
            notes: "Created via UniFFI demo",
            tags: [],
            totpSecret: nil,
            customFields: [],
            createdAt: now,
            modifiedAt: now,
            accessedAt: now,
            expiresAt: nil,
            isFavorite: false
        )
        
        do {
            _ = try vault.addEntry(entry: entry)
            // Save immediately for persistence
            try vault.save()
            loadEntries()
        } catch {
            print("Failed to add entry: \(error)")
        }
    }
}

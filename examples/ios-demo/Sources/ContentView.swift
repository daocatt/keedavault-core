import SwiftUI

struct ContentView: View {
    @StateObject private var viewModel = VaultViewModel()
    @State private var newPassword = ""
    
    var body: some View {
        NavigationView {
            VStack {
                if viewModel.isVaultOpen {
                    List(viewModel.entries, id: \.id) { entry in
                        VStack(alignment: .leading) {
                            Text(entry.title).font(.headline)
                            Text(entry.username).font(.subheadline)
                        }
                    }
                    
                    HStack {
                        TextField("New Password", text: $newPassword)
                            .textFieldStyle(RoundedBorderTextFieldStyle())
                            .autocapitalization(.none)
                        
                        Button("Add") {
                            viewModel.addEntry(password: newPassword)
                            newPassword = ""
                        }
                        .disabled(newPassword.isEmpty)
                    }.padding()
                } else {
                    VStack(spacing: 20) {
                        Text("KeedaVault Core Demo")
                            .font(.largeTitle)
                            .padding()
                        
                        Button("Create/Open Demo Vault") {
                            viewModel.setupDemoVault()
                        }
                        .padding()
                        .buttonStyle(.borderedProminent)
                        .controlSize(.large)
                        
                        Text("This will create a 'demo.kdbx' in Documents using password 'demo'")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                }
            }
            .navigationTitle(viewModel.isVaultOpen ? "My Vault" : "Welcome")
        }
    }
}

import SwiftUI

struct DevicesView: View {
    let accountId: Int32

    @EnvironmentObject var appState: AppState
    @State var account: Account?
    @State var devices: [Device] = []

    var body: some View {
        ZStack {
            if let account = account {
                _DevicesView(account: account, devices: devices) { device in
                    devices = devices.filter { d in d.id != device.id }
                }
            }
        }
        .onAppear {
            Task {
                switch await AccountViewModel.account(accountId) {
                case .success(let a): account = a
                case .failure(let e): appState.handleError(e)
                }
            }

            Task {
                switch await AccountViewModel.devices(accountId) {
                case .success(let d): devices = d
                case .failure(let e): appState.handleError(e)
                }
            }
        }
    }
}

private struct _DevicesView: View {
    let account: Account
    let devices: [Device]
    let onRemoveDevice: (Device) -> ()
    @EnvironmentObject var appState: AppState
    @State var deviceToRemove: Device?
    @State var showRemoveDevice = false
    @State var inProgress = false

    var body: some View {
        VStack {
            if devices.isEmpty {
                Text("There is no other device for this account. You can add new devices.")
                    .padding(.all, 12)
            }

            List(devices, id: \.id) { device in
                VStack(alignment: .leading, spacing: 8.0) {
                    Text(device.pubkey)
                    Text("Device is added at \(device.createdAt)")
                        .font(.subheadline)
                        .foregroundColor(.gray)
                }
                .swipeActions {
                    Button(action: {
                        deviceToRemove = device
                        showRemoveDevice =  true
                    }) {
                        Image(systemName: "trash.fill")
                    }
                    .tint(.red)
                }
            }
            .listStyle(.plain)

            Spacer()

            HStack {
                Spacer()
                NavigationLink(destination: DeviceAddView(accountId: account.id)) {
                    Image(systemName: "plus.circle")
                        .padding(EdgeInsets(top: 2, leading: 12, bottom: 12, trailing: 24))
                        .foregroundColor(.blue)
                }
            }
        }
        .navigationTitle("Devices")
        .alert(
            "Are you sure about removing device?",
            isPresented: $showRemoveDevice,
            actions: {
                Button("Remove", role: .destructive) {
                    if inProgress {
                        return
                    }

                    inProgress = true

                    Task {
                        let device = deviceToRemove!
                        switch await AccountViewModel.deleteDevice(account.id, device.id) {
                        case .success(_):
                            appState.emit(.ShowMessage("Device is removed"))
                            onRemoveDevice(device)
                        case .failure(let e): appState.handleError(e)
                        }

                        inProgress = false
                    }

                }
            },
            message: {
                Text("Removed device will not be able to access the account's notes and folders anymore. Removing a device will also cause any non synced notes and folders on the device to be lost.")
            }
        )
    }
}

struct Devices_Preview: PreviewProvider {
    static var previews: some View {
        NavigationView {
            _DevicesView(
                account: Account(id: 1, name: "Account", kind: .Local),
                devices: [Device(id: 1, accountId: 1, pubkey: "PUBKEY", createdAt: "2022-12-12")],
                onRemoveDevice: { _ in }
            )
        }
    }
}

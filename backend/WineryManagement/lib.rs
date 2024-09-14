#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod WineryManagement {
    use ink::prelude::string::String;
    use ink::storage::Mapping;

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct Winery {
        wineryID: u128,
        wineryName: String,
        wineryAddress: AccountId,
        latitude: i128,
        longitude: i128,
    }

    #[ink(event)]
    pub struct WineryCreated {
        ID: u128,
        wineryName: String,
        wineryAddress: AccountId,
        latitude: i128,
        longitude: i128,
        // wineryNumber: u128,
    }

    #[ink(storage)]
    pub struct WineryManagement {
        currentWineryID: u128,
        wineries: Mapping<(AccountId, u128), Winery>,
        userWineryCount: Mapping<AccountId, u128>,
    }

    impl WineryManagement {

        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                currentWineryID: Default::default(),
                wineries: Mapping::default(),
                userWineryCount: Mapping::default(),
            }
        }

        #[ink(message)]
        pub fn create_winery(&mut self, _name: String, _latitude: i128, _longitude: i128){
            // let winery_number: u128 = self.userWineryCount.insert(self.env().caller(), &1).expect("Couldnt add the number");

            self.currentWineryID = self.currentWineryID.checked_add(1).unwrap();

            let new_winery = Winery {
                wineryID: self.currentWineryID,
                wineryName: _name,
                wineryAddress: self.env().caller(),
                latitude: _latitude,
                longitude: _longitude,
            };

            self.wineries.insert((self.env().caller(), self.currentWineryID), &new_winery);
            self.env().emit_event(WineryCreated {
                ID: self.currentWineryID,
                wineryName: new_winery.wineryName,
                wineryAddress: self.env().caller(),
                latitude: new_winery.latitude,
                longitude: new_winery.longitude,
                // wineryNumber: winery_number,
            });
        }
        
        #[ink(message)]
        pub fn get_winery(&self, user_addy: AccountId, winery_number: u128) -> Option<Winery> {
            let winery = self.wineries.get((user_addy, winery_number));
            winery
        }
    }

    #[cfg(test)]
    mod tests {
  
    }


}

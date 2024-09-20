#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod winery_management {
    use ink::prelude::string::String;
    use ink::storage::Mapping;

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct Winery {
        winery_id: u128,
        winery_name: String,
        winery_address: AccountId,
        latitude: i128,
        longitude: i128,
    }

    #[ink(event)]
    pub struct WineryCreated {
        winery_id: u128,
        winery_name: String,
        winery_address: AccountId,
        latitude: i128,
        longitude: i128,
        winery_number: u128,
    }

    #[ink(storage)]
    pub struct WineryManagement {
        current_winery_id: u128,
        wineries: Mapping<(AccountId, u128), Winery>,
        user_winery_count: Mapping<AccountId, u128>,
    }

    impl WineryManagement {

        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                current_winery_id: 1,
                wineries: Mapping::default(),
                user_winery_count: Mapping::default(),
            }
        }

        #[ink(message)]
        pub fn create_winery(&mut self, _name: String, _latitude: i128, _longitude: i128){
            let mut winery_number = self.user_winery_count.get(self.env().caller()).unwrap_or(0);
            winery_number = winery_number.checked_add(1).unwrap();
            self.user_winery_count.insert(self.env().caller(), &winery_number);
            
            let new_winery = Winery {
                winery_id: self.current_winery_id,
                winery_name: _name,
                winery_address: self.env().caller(),
                latitude: _latitude,
                longitude: _longitude,
            };

            self.wineries.insert((self.env().caller(), &winery_number), &new_winery);
            self.env().emit_event(WineryCreated {
                winery_id: self.current_winery_id,
                winery_name: new_winery.winery_name,
                winery_address: self.env().caller(),
                latitude: new_winery.latitude,
                longitude: new_winery.longitude,
                winery_number,
            });
          
            self.current_winery_id = self.current_winery_id.checked_add(1).unwrap();
        }
        
        #[ink(message)]
        pub fn get_winery(&self, user_addy: AccountId, winery_number: u128) -> Option<Winery> {
            self.wineries.get((user_addy, winery_number))
        }
    }

    #[cfg(test)]
    mod tests {

        use super::*;

        #[ink::test]
        fn check_default_contract(){
            let winery_management = WineryManagement::default();
            assert_eq!(winery_management.current_winery_id, 1);

         
            let random_account = AccountId::from([0x1; 32]);
            assert_eq!(winery_management.user_winery_count.get(random_account).unwrap_or(0), 0);

            assert_eq!(winery_management.get_winery(random_account, 1), None);

        }



    }


}

#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod batch_management {
    use ink::prelude::string::String;
    use ink::storage::Mapping;


    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct Batch {
        batch_id: u128,
        batch_name: String,
        batch_owner: AccountId,
        temperature: Vec<u16>,
        wind_speed: Vec<u16>,
        rain_fall: Vec<u16>,
        soil_conductivity: Vec<u16>,
        humidity: Vec<u16>,
        removed: bool,
        batch_count: u128,
    }

    #[derive(Clone, Copy)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        WineryNotCreated,
        IncorrectAccess,
        BatchUnavailable,
    }

    #[ink(event)]
    pub struct BatchCreated {

    }

    #[ink(event)]
    pub struct WindDataPushed {

    }

    #[ink(event)]
    pub struct RainDataPushed {
        
    }

    #[ink(event)]
    pub struct SoilDataPushed {
        
    }

    #[ink(event)]
    pub struct HumidityDataPushed {
        
    }

    #[ink(event)]
    pub struct BatchRemoved {
        
    }

    #[ink(storage)]
    pub struct BatchManagement {
        current_batch_id: u128,
        batches: Mapping<(AccountId, u128), Batch>,
        user_batch_count: Mapping<AccountId, u128>,
    }

    impl BatchManagement {
      
        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                current_batch_id: Default::default(),
                batches: Mapping::default(),
                user_batch_count: Mapping::default(),
            }
        }

        #[ink(message)]
        pub fn create_batch(&mut self) {
            
        }


        #[ink(message)]
        pub fn push_data(&mut self) {
            
        }


        #[ink(message)]
        pub fn get_batch(&self) {
            
        }
    }

}

 // https://redux.js.org/usage/nextjs

import {configureStore} from '@reduxjs/toolkit'

// Place for all the slices/states. Creates a store instance
// per request. 
export const makeStore = () => {
   return configureStore({
    reducer: {}   
   })
}
export type AppStore = ReturnType<typeof makeStore> 
export type RootState = ReturnType<AppStore['getState']> // Get state from store
export type AppDispatch = AppStore['dispatch'] // Send to the store
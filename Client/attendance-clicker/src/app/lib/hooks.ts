import { useDispatch, useSelector, useStore } from "react-redux"
import type { AppDispatch, AppStore, RootState } from "./store"

// sends action to redux store. describes a change that should be made
// to the state. Store uses reducers to process actions and update
// state accordingly. useDispatch provides access to it
export const useAppDispatch = useDispatch.withTypes<AppDispatch>()

// reads/selects specific part of the state from redux store. useSelector provides
// access to it. specifies part of the state you are interested in. 
export const useAppSelector = useSelector.withTypes<RootState>()
export const useAppStore = useStore.withTypes<AppStore>()

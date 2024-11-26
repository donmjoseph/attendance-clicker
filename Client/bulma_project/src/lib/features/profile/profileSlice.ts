import { createSlice, PayloadAction } from "@reduxjs/toolkit";

// Profile slice

// Profile state
interface ProfileState {
    firstName: string;
    lastName: string;
    email: string;
    college: string;
    studentID: number;
}

// Set initial states
const initialState: ProfileState = {
    firstName: '',
    lastName: '',
    college: '',
    email: '',
    studentID: 23192849,
};

// Reducers
const profileSlice = createSlice({
    name: 'profile',
    initialState,
    reducers: {
        setFirstName(state, action: PayloadAction<string>) { 
            state.firstName = action.payload;
        },
        setLastName(state, action: PayloadAction<string>) {
            state.lastName = action.payload;
        },
        setEmail(state, action: PayloadAction<string>) {
            state.email = action.payload;
        },
        setStudentId(state, action: PayloadAction<number>) {
            state.studentID = action.payload;
        },
        setCollege(state, action: PayloadAction<string>) {
            state.college = action.payload;
        },

    },
});

// exports actions to use in app
export const {setFirstName, setLastName, setEmail, setStudentId, setCollege} = profileSlice.actions; 
export default profileSlice.reducer; // exports reducer to handle actions
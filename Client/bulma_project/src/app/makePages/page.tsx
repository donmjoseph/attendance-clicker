'use client';
import React, {useState} from 'react';
import { Provider, UseSelector, useDispatch, useSelector } from 'react-redux';
import { RootState, AppDispatch } from '@/lib/store';
import { makeStore } from '@/lib/store';
import axios from 'axios';
import StoreProvider from '@/StoreProvider';
import { setFirstName, setLastName } from '@/lib/features/profile/profileSlice'; 
import { useRouter, useSelectedLayoutSegment } from "next/navigation";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faCircle } from '@fortawesome/free-solid-svg-icons';
import { faBars } from '@fortawesome/free-solid-svg-icons';
import { library } from '@fortawesome/fontawesome-svg-core';
import { profile } from 'console';

library.add(faCircle);
library.add(faBars);

// Question Data format
interface QuestionData {
    question_id: number;
    class_id: string;
    class_name: string;
    question_title: string;
    answers: number[];
    correct_answer: number;
    created: string;
};

export default function Home() {
    const [menu, showMenu] = useState(false);

    const store = makeStore();

    const profileState = useSelector((state: RootState) => state.profile);
    const dispatch = useDispatch<AppDispatch>();

    const router = useRouter();

    const goToWaitingRoom = () => {
      router.push('/CheckedIn');
    }  

    const goToProfiles = () => {
        router.push('/profiles');
    }  
  
    const goToGrades = () => {
        router.push('/studentgrade');
    }  

    const showDropdownItems = () => {
        showMenu(!menu);
    };

    const checkDatabase = async () => {
        try {
            const response = await fetch('http://HIDDEN');  // localhost should work, but try an ip address instead
            if (!response.ok) {
                throw new Error('Network response was not ok');
            }
            const data = await response.json();
            console.log(data); // Log the response data
        } catch (error) {
            console.error('There was a problem with the fetch operation:', error);
        }
    };

    const testGetReq = async () => {
          axios.get('http://HIDDEN')
        .then(response => {
          console.log(response.data); // Handle the response data
        })
        .catch(error => {
          console.error('Error fetching data:', error);
        });

    }

    const testSendQuestion = async () => {
        // Usual format yay!
        const question: QuestionData = {
            question_id: 3,
            class_id: "550e8400-e29b-41d4-a716-446655440000", 
            class_name: "Physics 101",
            question_title: "What is Butt first law?",
            answers: [1, 2, 3, 4],
            correct_answer: 2,
            created: new Date().toISOString().split("Z")[0],
        };

            await axios.post('http:/HIDDEN', question, {
                headers: {
                "Content-Type": "application/json",
                },
            })
            .then((response) => {
                console.log("Response: ", response.data);
            })
            .catch((error) => {
                console.log("Error: ", error.response);
            });
        }

  return (
        
        <div>
        <div>
            <p>
                Welcome: {profileState.firstName} {profileState.lastName}
            </p>
        </div>
        <div className="box custom-menu">
            <div className={`dropdown ${menu ? `is-active` : ''} is-right`} >
                <div className="dropdown-trigger" >
                        <button className="button" aria-haspopup="true" aria-controls="dropdown-menu" onClick={showDropdownItems}>   
                            <span>
                            <FontAwesomeIcon icon="bars"/>
                            </span>
                        </button>
                    <div className="dropdown-menu" id="dropdown-menu" role="menu">
                        <div className="dropdown-content">
                            <div className="box custom-box">
                                <button className="class-button" onClick={goToProfiles}>
                                    Profile
                                </button>
                                <button className="class-button">
                                    Settings
                                </button>
                                <button className="class-button"  onClick={goToGrades}>
                                    Grades
                                </button>
                                <button className="class-button">
                                    Log out
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div> 
    </div>
    <div className="box custom-box">
        <h2> In session</h2>
        <button className='button class-button' onClick={goToWaitingRoom}>
            <p> <FontAwesomeIcon icon="circle"/> CPTS 101 - Prof John Doe </p>
            <p>MWF 1:00 PM to 3:00 PM</p>
        </button>   
        <div>
        </div>
    <button className='button class-button'>
        <p>BIO 102 - Prof Jane Doe </p>
        <p>Tue/Thurs 3:00 PM to 5:00 PM</p>
    </button>    
    <button className='button class-button'>
        <p>PHYS 104 - Prof Jack Doe </p>
        <p>Tue/Thurs 11:00 AM to 9:00 PM</p>
    </button>    
    <button className='button class-button' onClick={checkDatabase}>
        <p>Test check database</p>
    </button>
    <button className='button class-button' onClick={testGetReq}>
        <p>Test Database URL</p>
    </button>    
    <button className='button class-button' onClick={testSendQuestion}>
        <p>Test Send Question</p>
    </button> 
    </div>
    </div>
    );
}
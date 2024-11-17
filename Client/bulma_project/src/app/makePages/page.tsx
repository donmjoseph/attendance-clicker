'use client';
import React, {useState} from 'react';
import { useRouter } from "next/navigation";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faCircle } from '@fortawesome/free-solid-svg-icons';
import { faBars } from '@fortawesome/free-solid-svg-icons';
import { library } from '@fortawesome/fontawesome-svg-core';

library.add(faCircle);
library.add(faBars);

export default function Home() {
    const [menu, showMenu] = useState(false);

    const showDropdownItems = () => {
        showMenu(!menu);
    };

  return (
        <div>
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
                            <a href="#" className="dropdown-item">Profile</a>
                            <a href="#" className="dropdown-item">Settings</a>
                            <a href="#" className="dropdown-item">Sign Out</a>
                        </div>
                    </div>
                </div>
            </div> 
    </div>
    <div className="box custom-box">
        <h2> In session</h2>
        <button className='button class-button'>
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
    <button className='button class-button'>
        <p>CHEM 103 - Prof Jame Doe </p>
        <p>MWF 11:00 AM to 12:00 PM</p>
    </button>    
    </div>
    </div>
    );
}
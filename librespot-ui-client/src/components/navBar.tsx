import { PropsWithChildren } from "react"
import { Dropdown } from 'react-bootstrap';
import { User } from "../types/user";
import PersonCircle from 'bootstrap-icons/icons/person-circle.svg?react';

export interface NavBarProps {
    user?: User | null;
}

const getInitials = (user: User | null | undefined) => {
    if (!user) return "";
    return user.display_name.charAt(0);
}

export const NavBar = (props: PropsWithChildren<NavBarProps>) => {
    return (
    <nav className="navbar navbar-expand-lg navbar-light bg-light d-flex">
        <a className='navbar-brand p-2' href='#'>Librespot-UI</a>
        <div className="ms-auto p-2">
            <ul className="navbar-nav me-auto">
            <li className="nav-item dropdown">
                <Dropdown>
                    {
                        props.user ? (
                        <Dropdown.Toggle variant="outline-light" id="dropdown-basic">
                            <div data-initials={getInitials(props.user)}></div>
                        </Dropdown.Toggle>
                        ) : (
                        <Dropdown.Toggle variant="outline-light" id="dropdown-basic">
                            <div><PersonCircle fill="black" width="30px" height="30px" /></div>
                        </Dropdown.Toggle>
                        )
                    }
                    {
                        props.user ? (
                        <Dropdown.Menu>
                            <Dropdown.Header>User</Dropdown.Header>
                            <Dropdown.Item>{props.user?.display_name}</Dropdown.Item>
                            <Dropdown.Divider></Dropdown.Divider>
                            <Dropdown.Item href="/api/auth/logout">Log Out</Dropdown.Item>
                        </Dropdown.Menu>
                        ) : (
                        <Dropdown.Menu>
                            <Dropdown.Header>Not Logged In</Dropdown.Header>
                            <Dropdown.Divider></Dropdown.Divider>
                            <Dropdown.Item href="/login">Log In</Dropdown.Item>
                        </Dropdown.Menu>
                        )
                    }
                </Dropdown>
            </li>
            </ul>
        </div>
        { props.children }
    </nav>
    );
}
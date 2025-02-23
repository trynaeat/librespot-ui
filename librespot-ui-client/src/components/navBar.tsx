import { PropsWithChildren } from "react"
import { Dropdown } from 'react-bootstrap';
import { User } from "../types/user";

export interface NavBarProps {
    user?: User;
}

export const NavBar = (props: PropsWithChildren) => {
    return (
    <nav className="navbar navbar-expand-lg navbar-light bg-light d-flex">
        <a className='navbar-brand p-2' href='#'>Librespot-UI</a>
        <div className="ms-auto p-2">
            <ul className="navbar-nav me-auto">
            <li className="nav-item dropdown">
                <Dropdown>
                    <Dropdown.Toggle variant="success" id="dropdown-basic">
                        Dropdown Button
                    </Dropdown.Toggle>
                    <Dropdown.Menu>
                        <Dropdown.Item href="#/action-1">Action</Dropdown.Item>
                        <Dropdown.Divider></Dropdown.Divider>
                        <Dropdown.Item href="#/action-2">Another Action</Dropdown.Item>
                    </Dropdown.Menu>
                </Dropdown>
            </li>
            </ul>
        </div>
        { props.children }
    </nav>
    );
}
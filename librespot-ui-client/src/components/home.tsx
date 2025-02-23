import { useUser } from "../hooks/user"
import { LibreButtons } from "./libreButtons"
import { LibreStatus } from "./libreStatus"
import { NavBar } from "./navBar"

export const Home = () => {
    const { data: user, error, status } = useUser();
    return (
        <>
        <NavBar user={user}></NavBar>
        {
            user ? (
                <>
                <div className='pb-3 pt-5'><LibreStatus></LibreStatus></div>
                <LibreButtons></LibreButtons>
                </>
            ) : (
                <span>Not Logged In!</span>
            )
        }
        </>
    )
}
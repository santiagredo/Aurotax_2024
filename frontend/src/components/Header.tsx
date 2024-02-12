import { Fragment, useContext, useState } from "react";
import { Link } from "react-router-dom";
import { PageContext } from "./Context";
import LoginModal from "./Login";
import { createPortal } from "react-dom";
import { contactUs, home, profile, tutorials } from "../utils/routes";

const logo = new URL("../../public/diamond.svg", import.meta.url).href;
const headerMenu = new URL("../../public/menu.svg", import.meta.url).href;
const closeIcon = new URL("../../public/close.svg", import.meta.url).href;

export default function Header() {
    const context = useContext(PageContext);
    const [sideMenu, setSideMenuOpen] = useState(false);

    const ejectFromSideMenu = () => {
        setSideMenuOpen(false);
    };

    const injectIntoLoginModal = () => {
        context.setLoginModalOpen(true);
    };

    return (
        <header style={{zIndex: 1000}} className="top-0 right-0 mx-auto sticky py-2 px-3 w-full max-w-screen-2xl flex justify-between bg-white border-b-2 border-black">
            <nav className="flex items-center justify-center gap-10">
                <div className="flex content-center items-center">
                    <Link to={home} className="hidden lg:block">
                        <figure>
                            <img alt="Icon" src={logo} className="w-5" />
                        </figure>
                    </Link>

                    <Link
                        to={home}
                        className="hover:underline text-xl"
                    >
                        AuroTax
                    </Link>
                </div>

                <Link
                    to={tutorials}
                    className="hidden lg:block text-xl hover:underline"
                >
                    Tutorials
                </Link>

                <Link
                    to={contactUs}
                    className="hidden lg:block ml-auto text-xl hover:underline"
                >
                    Contact us
                </Link>
            </nav>

            {!context.userDetails.userId && (
                <button
                    type="button"
                    onClick={injectIntoLoginModal}
                    className="hidden lg:block px-3 py-2 w-20 rounded-md bg-blue-600 text-md text-white hover:underline"
                >
                    Log in
                </button>
            )}

            {context.userDetails.userId != 0 && (
                <>
                    <h2>{context.userDetails.userName}</h2>
                    <Link to={profile} onClick={ejectFromSideMenu}>
                        Profile
                    </Link>
                    <button
                        type="button"
                        onClick={injectIntoLoginModal}
                        className="hidden lg:block px-3 py-2 w-20 rounded-md bg-blue-600 text-md text-white hover:underline hover:font-semibold"
                    >
                        Log out
                    </button>
                </>
            )}

            <button
                type="button"
                onClick={() => setSideMenuOpen(true)}
                className="lg:hidden"
            >
                <figure>
                    <img
                        alt="header menu icon"
                        src={headerMenu}
                        className="w-9"
                    />
                </figure>
            </button>

            {sideMenu && (
                <div className="top-0 left-0 w-screen h-screen absolute flex justify-end bg-black bg-opacity-50 lg:hidden">
                    <div
                        className="absolute inset-0 bg-black opacity-20"
                        onClick={ejectFromSideMenu}
                    ></div>
                    <nav className="z-10 py-2 px-3 w-screen max-w-xs h-screen flex flex-col items-start gap-1 border-1-2 border-black bg-white text-xl">
                        <button
                            type="button"
                            onClick={ejectFromSideMenu}
                            className="self-end"
                        >
                            <figure className="w-4 ml-auto">
                                <img alt="Close side menu" src={closeIcon} />
                            </figure>
                        </button>

                        <Link
                            to={home}
                            className="mt-2 mb-8"
                            onClick={ejectFromSideMenu}
                        >
                            AuroTax
                        </Link>

                        {context.userDetails.userId != 0 && (
                            <Fragment>
                                <h2>{context.userDetails.userName}</h2>
                                <Link to={profile} onClick={ejectFromSideMenu}>
                                    Profile
                                </Link>
                            </Fragment>
                        )}

                        <Link to={tutorials} onClick={ejectFromSideMenu}>
                            Tutorials
                        </Link>

                        <Link to={contactUs} onClick={ejectFromSideMenu}>
                            Contact us
                        </Link>

                        {!context.userDetails.userId && (
                            <button
                                type="button"
                                onClick={() => {
                                    ejectFromSideMenu();
                                    injectIntoLoginModal();
                                }}
                            >
                                Log in
                            </button>
                        )}
                    </nav>
                </div>
            )}

            {context.loginModal && createPortal(<LoginModal />, document.body)}
        </header>
    );
}

import { useContext, useState } from "react";
import { PageContext } from "./Context";
import { Link } from "react-router-dom";
import { apiUri } from "../settings";
import { userLogin } from "../utils/routes";
import ErrorModal from "./Error";
import { createPortal } from "react-dom";
import {
    COOKIE_USER_EMAIL,
    COOKIE_USER_ID,
    COOKIE_USER_NAME,
    getCookie,
} from "../utils/cookies";

const closeIcon = new URL("../../public/close.svg", import.meta.url).href;

export default function LoginModal() {
    const context = useContext(PageContext);

    const [loading, setLoading] = useState(false);
    const [errorState, setErrorState] = useState(false);

    const [userEmail, setUserEmail] = useState("");
    const handleEmailChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        setUserEmail(e.target.value);
    };

    const [userPassword, setUserPassword] = useState("");
    const handlePasswordChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        setUserPassword(e.target.value);
    };

    const handleFormSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();

        setLoading(true);
        setErrorState(false);

        const apiResponse = await sendLoginRequest(userEmail, userPassword);

        switch (apiResponse.status) {
            case 401:
                setErrorState(true);
                break;
            case 400:
                setErrorState(true);
                break;
            case 200:
                context.setUserDetails({
                    userEmail: getCookie(COOKIE_USER_EMAIL),
                    userId: Number(getCookie(COOKIE_USER_ID)) || 0,
                    userName: getCookie(COOKIE_USER_NAME),
                });

                ejectFromLoginModal();
                setErrorState(false);
                break;
            default:
                context.setErrorModalOpen(true);
                break;
        }

        setLoading(false);
    };

    const ejectFromLoginModal = () => {
        context.setLoginModalOpen(false);
    };

    const isSubmitButtonDisabled =
        loading || userEmail.length < 1 || userPassword.length < 7;

    return (
        <section style={{zIndex: 1000}} className="fixed inset-0 flex items-center justify-center">
            <div
                className="absolute inset-0 bg-black opacity-60"
                onClick={ejectFromLoginModal}
            ></div>

            <article className="w-screen max-w-xl h-screen max-h-96 p-4 bg-white rounded-md z-10">
                <div className="pb-3 flex items-center border-b-2 border-gray-200">
                    <button type="button"
                        className="w-4 flex hover:cursor-pointer"
                        onClick={ejectFromLoginModal}
                    >
                        <img alt="Close login modal" src={closeIcon} />
                    </button>
                    <h2 className="block mx-auto font-semibold">Log in</h2>
                </div>

                <h2 className="mt-4 text-2xl text-center font-semibold">
                    Welcome to AuroTax
                </h2>

                <p
                    className={`text-center text-red-500 ${
                        !errorState && "hidden"
                    }`}
                >
                    Please check your Email and Password
                </p>

                <form onSubmit={handleFormSubmit} className="flex flex-col">
                    <input
                        onChange={handleEmailChange}
                        placeholder="Email address"
                        type="email"
                        className={`h-14 mt-2 px-2 border-2 border-gray-200 rounded-md hover:bg-gray-100 ${
                            errorState && "border-red-500 bg-red-50"
                        }`}
                    />

                    <input
                        onChange={handlePasswordChange}
                        placeholder="Password"
                        type="password"
                        className={`h-14 mt-5 px-2 border-2 border-gray-200 rounded-md hover:bg-gray-100 ${
                            errorState && "border-red-500 bg-red-50"
                        }`}
                    />

                    <button
                        type="submit"
                        disabled={isSubmitButtonDisabled}
                        className={`h-14 mt-5 px-2 rounded-md bg-blue-600 text-white font-semibold hover:bg-blue-700 hover:underline ${
                            isSubmitButtonDisabled && "opacity-50"
                        }`}
                    >
                        Continue
                    </button>

                    <Link
                        to="/"
                        className="mt-2 text-center text-blue-400 hover:text-blue-600 hover:underline"
                    >
                        I forgot my password
                    </Link>
                </form>
            </article>

            {context.errorModal && createPortal(<ErrorModal />, document.body)}
        </section>
    );
}

interface ResponseData {
    status: number;
    message: string;
}

async function sendLoginRequest(userEmail: String, userPassword: String) {
    try {
        const userData = {
            email: userEmail,
            password: userPassword,
        };

        const response = await fetch(`${apiUri}${userLogin}`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(userData),
        });

        if (!response.ok) {
            throw response;
        }

        const responseData: ResponseData = {
            status: response.status,
            message: await response.json(),
        };

        return responseData;
    } catch (error) {
        if (error instanceof Response) {
            const responseData: ResponseData = {
                status: error.status,
                message: (await error.text())
                    ? JSON.parse(await error.text())
                    : "",
            };

            return responseData;
        }

        return { status: 500, message: "Internal Server Error" };
    }
}

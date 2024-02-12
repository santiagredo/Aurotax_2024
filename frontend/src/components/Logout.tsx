import { useContext, useState } from "react";
import { PageContext } from "./Context";
import { apiUri } from "../settings";
import { userLogout } from "../utils/routes";
import {
    COOKIE_USER_EMAIL,
    COOKIE_USER_ID,
    COOKIE_USER_NAME,
    deleteCookie,
    getCookie,
} from "../utils/cookies";

const closeIcon = new URL("../../public/close.svg", import.meta.url).href;

export default function LogoutModal() {
    const context = useContext(PageContext);

    const [loading, setLoading] = useState(false);

    const handleFormSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();

        setLoading(true);

        const apiResponse = await sendLogoutRequest();

        switch (apiResponse.status) {
            case 200:
                context.setUserDetails({
                    userEmail: getCookie(COOKIE_USER_EMAIL),
                    userId: Number(getCookie(COOKIE_USER_ID)) || 0,
                    userName: getCookie(COOKIE_USER_NAME),
                });

                break;
            default:
                context.setUserDetails({
                    userEmail: "",
                    userId: 0,
                    userName: "",
                });

                deleteCookie(COOKIE_USER_EMAIL);
                deleteCookie(COOKIE_USER_ID);
                deleteCookie(COOKIE_USER_NAME);

                break;
        }

        ejectFromLogoutModal();
        setLoading(false);
    };

    const ejectFromLogoutModal = () => {
        context.setLogoutModalOpen(false);
    };

    return (
        <section style={{zIndex: 1000}} className="fixed inset-0 flex items-center justify-center">
            <div
                className="absolute inset-0 bg-black opacity-60"
                onClick={ejectFromLogoutModal}
            ></div>

            <article className="w-screen max-w-xl h-screen max-h-96 p-4 bg-white rounded-md z-10">
                <div className="pb-3 flex border-b-2 border-gray-200">
                    <button
                        type="button"
                        className="w-4 flex hover:cursor-pointer"
                        onClick={ejectFromLogoutModal}
                    >
                        <img alt="Close logout modal" src={closeIcon} />
                    </button>
                    <h2 className="block mx-auto font-semibold">Log out</h2>
                </div>

                <div className="mt-12">
                    <h2 className="mt-4 text-2xl text-center font-semibold">
                        {context.userDetails.userName}
                    </h2>

                    <p className="mt-4 text-lg text-center">
                        Are you sure you want to close your session?
                    </p>
                </div>

                <form
                    onSubmit={handleFormSubmit}
                    className="mt-4 flex flex-col"
                >
                    <button
                        onClick={ejectFromLogoutModal}
                        type="button"
                        className={`h-14 mt-5 px-2 rounded-md bg-blue-600 text-white font-semibold hover:bg-blue-700 hover:underline ${
                            loading && "opacity-20"
                        }`}
                    >
                        Cancel
                    </button>

                    <button
                        type="submit"
                        className={`h-14 mt-5 px-2 rounded-md bg-red-600 text-white font-semibold hover:bg-red-700 hover:underline ${
                            loading && "opacity-20"
                        }`}
                    >
                        Logout
                    </button>
                </form>
            </article>
        </section>
    );
}

interface ResponseData {
    status: number;
    message: string;
}

async function sendLogoutRequest() {
    try {
        const response = await fetch(`${apiUri}${userLogout}`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
        });

        if (!response.ok) {
            console.log(response);
            throw response;
        }

        const responseData: ResponseData = {
            status: response.status,
            message: await response.json(),
        };
        console.log(responseData);
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

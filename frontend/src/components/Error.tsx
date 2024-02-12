import { useContext } from "react";
import { PageContext } from "./Context";

const closeIcon = new URL("../../public/close.svg", import.meta.url).href;
const errorIcon = new URL("../../public/error.svg", import.meta.url).href;

export default function ErrorModal() {
    const context = useContext(PageContext);

    const ejectFromErrorModal = () => {
        context.setErrorModalOpen(false);
    };

    return (
        <section className="z-50 fixed inset-0 flex items-center justify-center">
            <div
                className="absolute inset-0 bg-black opacity-60"
                onClick={ejectFromErrorModal}
            ></div>

            <article className="w-screen max-w-xl h-screen max-h-96 p-4 bg-white rounded-md z-10 bg-cover bg-center">
                <div className="pb-3 flex border-b-2 border-gray-200">
                    <figure
                        className="w-4 flex hover:cursor-pointer"
                        onClick={ejectFromErrorModal}
                    >
                        <img alt="Close login modal" src={closeIcon} />
                    </figure>
                    <h2 className="block mx-auto font-semibold">Error 500</h2>
                </div>

                <div className="h-52 flex flex-col justify-around place-items-center">
                    <figure className="absolute opacity-10">
                        <img
                            alt="Error icon"
                            src={errorIcon}
                            className="w-52"
                        />
                    </figure>

                    <h2 className="text-2xl text-center font-semibold">Ops!</h2>

                    <h2 className="text-2xl text-center font-semibold">
                        There was a problem processing your request
                    </h2>

                    <h2 className="mt-4 text-2xl text-center font-semibold">
                        Please try again in a moment
                    </h2>
                </div>

                <button onClick={ejectFromErrorModal} className="h-14 w-full mt-5 px-2 rounded-md bg-blue-600 text-white font-semibold hover:bg-blue-700 hover:underline">
                    Ok
                </button>
            </article>
        </section>
    );
}

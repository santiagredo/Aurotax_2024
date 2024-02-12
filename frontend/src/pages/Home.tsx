import { Link, useNavigate } from "react-router-dom";
import { OnApproveData } from "../utils/models";
import { PayPalButtons, PayPalScriptProvider } from "@paypal/react-paypal-js";

const chat = new URL("../../public/chat.png", import.meta.url).href;
const idea = new URL("../../public/idea.png", import.meta.url).href;
const care = new URL("../../public/care.png", import.meta.url).href;

export default function Home() {
    const navigate = useNavigate();

    function createOrder() {
        return fetch("http://127.0.0.1:8080/orders/create_order", {
            method: "POST",
            mode: "cors",
            headers: {
                "Content-Type": "application/json",
                "Access-Control-Allow-Credentials": "true",
            },
        })
            .then((response) => response.json())
            .then((order) => order.id);
    }

    function onApprove(data: OnApproveData) {
        console.log(data);

        return fetch(
            `http://127.0.0.1:8080/orders/capture_order/${data.orderID}`,
            {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                    "Access-Control-Allow-Credentials": "true",
                },
                mode: "cors",
                credentials: "include",
                body: JSON.stringify({
                    orderID: data.orderID,
                }),
            }
        )
            .then((response) => response.json())
            .then((orderData) => {
                const name = orderData.payer.name.given_name;
                console.log(orderData);
                alert(`Transaction completed by ${name}`);
                navigate(`/test`);
            });
    }

    return (
        <main className="w-full">
            <section>
                <div>
                    <h1 className="mt-5 text-3xl text-center font-semibold">
                        Welcome!
                    </h1>

                    <p className="mt-5 text-xl text-center">
                        We are here to help you and that's why we offer:
                    </p>
                </div>

                <div className="mx-auto w-5/6 flex flex-col lg:flex-row justify-between">
                    <article className="mt-5 mx-auto lg:mx-0 max-w-96 lg:max-h-96 flex flex-col border border-blue-500 rounded-lg">
                        <div
                            className="h-12 flex items-center justify-center content-center text-center bg-blue-500 text-white text-xl rounded-lg"
                            style={{
                                borderBottomLeftRadius: "0px",
                                borderBottomRightRadius: "0px",
                            }}
                        >
                            <h2>Aupair IRS</h2>
                        </div>

                        <ul className="w-4/6 mx-auto flex flex-col gap-2">
                            <li className="mt-5 text-center text-2xl font-semibold">
                                Free
                            </li>
                            <li className="mt-3">
                                Completion of form 1040 and Annexes
                            </li>
                            <li>
                                Accompaniment in the process (Applicable
                                conditions)
                            </li>
                            <li>Immediate download</li>
                            <li>Shipping and payment guide</li>
                        </ul>

                        <Link
                            to="/FreeTier"
                            className="w-5/6 my-5 mx-auto px-2 h-12 flex items-center content-center justify-center bg-blue-500 text-white text-center text-lg rounded-md leading-5"
                        >
                            Start the Aupair IRS process for free
                        </Link>
                    </article>

                    <article className="mt-5 mx-auto lg:mx-0 w-full max-w-96 flex flex-col border border-green-600 rounded-lg">
                        <div
                            className="h-12 flex items-center justify-center content-center text-center bg-green-600 text-white text-xl rounded-lg"
                            style={{
                                borderBottomLeftRadius: "0px",
                                borderBottomRightRadius: "0px",
                            }}
                        >
                            <h2>Self-Employment (SE)</h2>
                        </div>

                        <ul className="w-4/6 mx-auto flex flex-col gap-2">
                            <li className="mt-5 text-center text-2xl font-semibold">
                                $35.99
                            </li>
                            <li className="mt-3">
                                Filling out form 1040 for SE
                            </li>
                            <li>Immediate download</li>
                            <li>Full support throughout the process</li>
                        </ul>

                        <div className="mt-5 w-5/6 mx-auto">
                            <PayPalScriptProvider
                                options={{
                                    clientId:
                                        "AQVWuxiXYYCHnwvtr66T_WX15nD96SGnN9pNMwqerFJ17ONgXEFPlJbFE0CAapwjyFDbCQjpbW6NcZKh",
                                }}
                            >
                                <PayPalButtons
                                    style={{ layout: "vertical" }}
                                    createOrder={createOrder}
                                    onApprove={onApprove}
                                />
                            </PayPalScriptProvider>
                        </div>
                    </article>
                </div>
            </section>

            <section className="mt-10 flex flex-col items-center">
                <h2 className="px-2 text-center text-xl font-semibold">
                    Get Self-Employment (SE) forms in three simple steps
                </h2>

                <div className="mt-5 mx-auto w-5/6 flex flex-col lg:flex-row items-center gap-8 lg:gap-16">
                    <article
                        style={{ borderColor: "#247BA0" }}
                        className="h-48 w-64 flex flex-col items-center border rounded-lg"
                    >
                        <div
                            style={{
                                backgroundColor: "#247BA0",
                                borderBottomLeftRadius: "0px",
                                borderBottomRightRadius: "0px",
                            }}
                            className="w-full h-12 rounded-lg flex justify-center items-center text-white"
                        >
                            <label className="font-semibold text-lg">
                                Step 1
                            </label>
                        </div>
                        <p
                            style={{ color: "#247BA0" }}
                            className="my-auto font-semibold text-lg"
                        >
                            Make a payment
                        </p>
                    </article>

                    <article
                        style={{ borderColor: "#32A287" }}
                        className="h-48 w-64 flex flex-col items-center border rounded-lg"
                    >
                        <div
                            style={{
                                backgroundColor: "#32A287",
                                borderBottomLeftRadius: "0px",
                                borderBottomRightRadius: "0px",
                            }}
                            className="w-full h-12 rounded-lg flex justify-center items-center text-white"
                        >
                            <label className="font-semibold text-lg">
                                Step 2
                            </label>
                        </div>
                        <p
                            style={{ color: "#32A287" }}
                            className="my-auto font-semibold text-lg"
                        >
                            Complete the form
                        </p>
                    </article>

                    <article
                        style={{ borderColor: "#706993" }}
                        className="h-48 w-64 flex flex-col items-center border rounded-lg"
                    >
                        <div
                            style={{
                                backgroundColor: "#706993",
                                borderBottomLeftRadius: "0px",
                                borderBottomRightRadius: "0px",
                            }}
                            className="w-full h-12 rounded-lg flex justify-center items-center text-white"
                        >
                            <label className="font-semibold text-lg">
                                Step 3
                            </label>
                        </div>
                        <p
                            style={{ color: "#706993" }}
                            className="my-auto font-semibold text-lg"
                        >
                            Download the form
                        </p>
                    </article>
                </div>
            </section>

            <section className="mt-10 w-full flex flex-col">
                <h2 className="px-2 text-center text-xl font-semibold">
                    Some of the many reasons to trust us
                </h2>

                <article
                    style={{ maxWidth: "896px" }}
                    className="relative mt-5 py-4 px-1 mx-auto w-5/6 flex flex-col lg:flex-row gap-6 text-center border-2 rounded-lg"
                >
                    <div className="mx-auto flex flex-col gap-5">
                        <h3 className="text-xl font-semibold">
                            Say goodbye to the complicated
                        </h3>

                        <p>
                            Forget about how complicated "Taxes" are. We
                            accompany you step by step
                        </p>
                    </div>

                    <figure className="absolute w-32 h-full mx-auto inset-0">
                        <img src={chat} className="opacity-10 h-full" />
                    </figure>
                </article>

                <article
                    style={{ maxWidth: "896px" }}
                    className="relative mt-5 py-4 px-1 mx-auto w-5/6 flex flex-col lg:flex-row gap-6 text-center border-2 rounded-lg"
                >
                    <div className="mx-auto flex flex-col gap-5">
                        <h3 className="text-xl font-semibold">
                            We provide free solutions to the community
                        </h3>

                        <p>Knowledge is to share</p>

                        <Link to="/FreeTier">Aupair-IRS is Free!</Link>
                    </div>

                    <figure className="absolute w-32 h-full mx-auto inset-0">
                        <img src={idea} className="opacity-10 h-full" />
                    </figure>
                </article>

                <article
                    style={{ maxWidth: "896px" }}
                    className="relative mt-5 py-4 px-1 mx-auto w-5/6 flex flex-col lg:flex-row gap-6 text-center border-2 rounded-lg"
                >
                    <div className="mx-auto flex flex-col gap-5">
                        <h3 className="text-xl font-semibold">
                            Adult life is not easy, much less when you have to
                            pay taxes!
                        </h3>

                        <p>
                            That is why we guide you to take care of every penny
                        </p>
                    </div>

                    <figure className="absolute w-32 h-full mx-auto inset-0">
                        <img src={care} className="opacity-10 h-full" />
                    </figure>
                </article>

                <div className="w-5/6 mx-auto flex flex-col md:flex-row gap-x-3">
                    <article className="relative mt-5 py-4 px-1 mx-auto w-full flex flex-col gap-6 justify-between text-center border-2 rounded-lg">
                        <h3 className="text-xl font-semibold">
                            Aupair IRS What does the IRS say about the AUPAIR
                            program?
                        </h3>

                        <p>
                            In the United States, the term has come to have a
                            narrow technical meaning describing a class of
                            Exchange Visitors who come to the United States
                            under the auspices of a program initially
                            administered by the U.S. Agency for...
                        </p>

                        <Link to="https://www.irs.gov/individuals/international-taxpayers/au-pairs">
                            Read more
                        </Link>
                    </article>

                    <article className="relative mt-5 py-4 px-1 mx-auto w-full flex flex-col gap-6 justify-between text-center border-2 rounded-lg">
                        <h3 className="text-xl font-semibold">
                            What is Self-Employment Tax?
                        </h3>

                        <p>
                            The self-employment tax is a tax that consists of
                            Social Security and Medicare taxes and is primarily
                            intended for self-employed people. Is similar to...
                        </p>
                        <Link to="https://www.irs.gov/individuals/international-taxpayers/au-pairs">
                            Read more
                        </Link>
                    </article>
                </div>
            </section>

            <section className="my-10 w-full flex flex-col">
                <h2 className="px-2 text-center text-xl font-semibold">
                    Information of interest
                </h2>

                <div className="mx-auto flex flex-col md:flex-row gap-x-10">
                    <article className="mx-auto mt-5 w-5/6 flex flex-col">
                        <h3 className="text-center font-semibold">Resources</h3>

                        <ul className="mt-2 flex flex-col gap-y-1">
                            <Link to="">Aupair IRS</Link>
                            <Link to="">What is Self-Employment Tax?</Link>
                            <Link to="">Get SSN</Link>
                            <Link to="">Make tax payment - Pay1040</Link>
                            <Link to="">Where to send my taxes?</Link>
                            <Link to="">About 1040 form</Link>
                            <Link to="">
                                Request your tax certificate - IRS
                            </Link>
                        </ul>
                    </article>

                    <article className="mx-auto mt-5 w-5/6">
                        <div className="flex flex-col gap-y-3">
                            <h3 className="text-center font-semibold">Legal</h3>

                            <ul className="flex flex-col gap-y-1">
                                <Link to="">Terms and conditions</Link>
                                <Link to="">Data processing policy</Link>
                                <Link to="">
                                    Notice for the processing of personal data
                                </Link>
                            </ul>
                        </div>

                        <div className="mt-4 flex flex-col gap-y-3">
                            <h3 className="text-center font-semibold">Get in touch</h3>

                            <Link to="/Contact">Contact us</Link>
                        </div>
                    </article>
                </div>
            </section>
        </main>
    );
}

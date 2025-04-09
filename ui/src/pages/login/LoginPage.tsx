import { useState } from "react"
import { RegularText } from "../../utils/theming/RegularText";
import { Button } from "../../utils/theming/Button";
import { Input } from "../../utils/theming/Input";
import { ProtectedInput } from "../../utils/theming/ProtectedInput";
import { Center } from "../../utils/theming/Center";
import { BigText } from "../../utils/theming/BigText";

export const LoginPage = () => {

    const [login, setLogin] = useState("");
    const [password, setPassword] = useState("")

    const loginAction = async () => {
        
    }

    return (
        <Center>
            <div className="flex flex-col space-y-8 items-center bg-foreground p-16 rounded-lg shadow-sm">
                <div className="mx-auto">
                    <BigText text="Crypto scam" />
                </div>
                <div className="mx-auto px-4">
                    <Input placeholder="Enter login" value={login} onChange={setLogin} />
                </div>
                <div className="mx-auto px-4">
                    <ProtectedInput placeholder='Enter password' value={password} onChange={setPassword} />
                </div>
                <div className="flex flex-row space-x-4">
                    <Button>
                        <RegularText text={"Login"} />
                    </Button>
                </div>
            </div>
        </Center>
    )
}
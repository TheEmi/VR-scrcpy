import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./css/Single.css";

interface SingleProps {
    addresses: string[] | undefined;
}

function Single({ addresses }: SingleProps) {
    const [options, setOptions] = useState<string[]>();
    useEffect(() => {
        let all_addresses: string[] = [];
        //maybe filter
        addresses?.forEach((address) => {
            console.log(address.split("\t")[0])
            all_addresses.push(address.split("\t")[0]);
        })
        setOptions(all_addresses);
    }, [addresses])

  function launch_app() {
    invoke("launch_app").then((res) => {
      console.log(res)
    })
  }

  return (
    <div className="container">
      <h3>Single</h3>
      <select name="address" id="address">
        {options?.map((option) => {return <option value={option}>{option}</option>})}
      </select>
      <button onClick={launch_app}>Launch</button>

    </div>
  );
}

export default Single;

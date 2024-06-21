import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./css/App.css";
import {
  Button,
  Card,
  Col,
  Input,
  Row,
  Select,
  Space,
  Table,
} from "antd";
import { z } from "zod";

type IpType = {
  key: React.Key;
  ip: string;
  status: string;
};

type SelectedType = {
  label: string;
  value: string;
};

function App() {
  const [all_addresses, setAllAddresses] = useState<IpType[]>([]);
  const [selected_addresses, setSelectedAddresses] = useState<IpType[]>([]);
  const [selectedProcess, setSelectedProcess] = useState("");
  const [ipInput, setIpInput] = useState("");
  const [appName, setAppName] = useState("");
  const [appsLoading, setAppsLoading] = useState(true);
  const [apps, setApps] = useState<SelectedType[]>([]);
  const [startLoading, setStartLoading] = useState(false);
  const [stopLoading, setStopLoading] = useState(false);
  const [connectLoading, setConnectLoading] = useState(false);
  useEffect(() => {
    handle_get_devices();
    get_processes();
  },[]);

  function get_processes(){
    setAppsLoading(true);
    invoke("get_processes").then((res) => {
      const response = z.string().array().parse(res);
      console.log(response);
      const newApps = response.map((app) => {
        return { label: app, value: app };
      }
      );
      setApps(newApps);
      setAppsLoading(false);
    });
  }

  function insert_ip(ip: string) {
    invoke("insert_new_ip", { ip: ip });
  }

  function insert_app(appName: string) {
    invoke("insert_new_process", { appName: appName });
    get_processes();

  }
  function handle_get_devices() {
    invoke("get_connected_devices").then((res) => {
      const responsee = z.string().array().parse(res);
      let newList: IpType[] = [];
      let counter = 0;
      responsee.forEach((element) => {
        newList.push({ key: counter, ip: element.split("\t")[0], status: element.split("\t")[1] });
        counter++;
      });
      setAllAddresses(newList);
    });
  }
  function handle_connect_devices() {
    setConnectLoading(true);
    invoke("connect_to_ips").then((res) => {
      console.log(res);
      setConnectLoading(false)
    });
  }

  function handle_disconnect(){
    let new_addresses = selected_addresses.map((address) => address.ip);
    invoke("disconnect_multiple_devices", {addresses: new_addresses}).then((res) => {
      console.log(res);
      handle_get_devices();
    });
  }

  function delay(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
  }
  async function handle_scrcpy() {
    let new_addresses = selected_addresses.map((address) => address.ip);
  
    for (const address of new_addresses) {
      invoke("scrcpy_device", { adress: address }).then((res) => {
        console.log(res);
      });
      await delay(3000);  // Wait for 3 seconds before the next call
    }
  }

  function launch_apps() {
    let new_addresses = selected_addresses.map((address) => address.ip);
    invoke("launch_app_multiple", {addresses: new_addresses,process: selectedProcess}).then((res) => {
      console.log(res);
    });
  }
  function close_apps() {
    let new_addresses = selected_addresses.map((address) => address.ip);
    invoke("close_app_multiple", {addresses: new_addresses,process: selectedProcess.split("/")[0]}).then((res) => {
      console.log(res);
    });
  }

  function handle_start() {
    setStartLoading(true);
    invoke("start_server", { var: "a" }).then((res) => {
      console.log(res);
      setStartLoading(false);
    });
  }

  function handle_stop() {
    setStopLoading(true);
    invoke("stop_server", { var: "a" }).then((res) => {
      console.log(res);
      setStopLoading(false);
    });
  }

  const get_color = (status: string) => {
      switch(status){
      case "device": 
        return "green";
      case "offline":
        return "red";
      default:
        return "yellow";
    }
  }

  const rowSelection = {
    //-- @ts-ignore
    onChange: (_: React.Key[], selectedRows: IpType[]) => {
      setSelectedAddresses(selectedRows);
    },
  };

  return (
    <div>
      <Card style={{ height: "80vh" }}>
        <Row>
          <Col style={{ width: "30%" }}>
            <Table
              dataSource={all_addresses}
              rowSelection={{
                type: "checkbox",
                ...rowSelection,
              }}
              pagination={false}
            >
              <Table.Column
                title="IP"
                render={(_, record: any, __) => {
                  return record.ip;
                }}
              ></Table.Column>
              <Table.Column
                title="Status"
                render={(_, record: any, __) => {
                  return <div style={{width: "20px", height: "20px", backgroundColor: get_color(record.status)}}></div>;
                }}
              ></Table.Column>
            </Table>
          </Col>
          <Col style={{ width: "70%" }}>
            <div style={{ margin: "15px", display: "grid", gap: "40%" }}>
              <Select options={apps} loading={appsLoading} onChange={(e)=>{setSelectedProcess(e)}}></Select>
              <Row justify="space-between">
                <Button style={{ width: "45%" }} type="primary" onClick={launch_apps}>
                  Launch
                </Button>
                <Button style={{ width: "45%" }} danger type="primary" onClick={close_apps}>
                  Close
                </Button>
              </Row>
              <Button danger onClick={handle_disconnect}>Disconnect</Button>
              <Button type="primary" onClick={handle_scrcpy}>Launch SCRCPY</Button>
            </div>
          </Col>
        </Row>
      </Card>
      <Card style={{ height: "14vh" }}>
        <Row style={{ display: "flex", justifyContent: "space-between" }}>
          <Space.Compact>
            <Button type="primary" onClick={handle_get_devices}>
              Get All
            </Button>
            <Button type="primary" onClick={handle_connect_devices} loading={connectLoading}>
              Connect
            </Button>
          </Space.Compact>
          <Space.Compact>
            <Input
              placeholder="192.168.X.X"
              onChange={(e) => setIpInput(e.target.value)}
            />
            <Button type="primary" onClick={() => insert_ip(ipInput)}>
              Add IP
            </Button>
          </Space.Compact>
          <Space.Compact>
            <Input placeholder="com.beatsaber" 
            onChange={(e) => setAppName(e.target.value)}/>
            <Button type="primary" onClick={() => insert_app(appName)}>Add App</Button>
          </Space.Compact>
          <Space.Compact>
            <Button
              type="primary"
              loading={startLoading}
              onClick={handle_start}
            >
              Start
            </Button>
            <Button
              danger
              type="primary"
              loading={stopLoading}
              onClick={handle_stop}
            >
              Stop
            </Button>
          </Space.Compact>
        </Row>
      </Card>
    </div>
  );
}

export default App;

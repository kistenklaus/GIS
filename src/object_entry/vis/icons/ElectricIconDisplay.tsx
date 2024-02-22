import { Bolt } from "@mui/icons-material";
import { Box, Typography } from "@mui/material";
import { ObjectEntryListenLatestResponse } from "../../types/events/ObjectEntryListenLatestResponse";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { ObjectEntryEvent } from "../../types/events/ObjectEntryEvent";


const OE = {nodeName : "master", objectEntryName: "sdc_status"};

function ElectricIconDisplay() {

  const [state, setState] = useState<boolean>(false);

  useEffect(()=>{
    async function asyncSetup() {
      const resp = await invoke<ObjectEntryListenLatestResponse>("listen_to_latest_object_entry_value", OE);
      if (resp.latest !== undefined && resp.latest !== null) {
        setState(resp.latest.value as string == "CLOSED");
      }
      const unlisten = await listen<ObjectEntryEvent>(resp.event_name, event => {
          setState(event.payload.value as string == "CLOSED");
      });
      return () => {
        unlisten();
        invoke("unlisten_from_latest_object_entry_value", OE).catch(console.error);
      };
    }
    const asyncCleanup = asyncSetup();

    return () => {
      asyncCleanup.then(f=>f()).catch(console.error);
    };

  },[]);

  //TODO use theme for colors
  return (
    <Box component="form" sx={{
      textAlign: "center",
    }}>
      <Bolt id="electric-icon" sx={{ fontSize: "42px", color: state ? "red" : "grey"}} />
      <div style={{ marginBottom: "-6px" }} />
      <Typography color="black">
        Electric
      </Typography>
    </Box>
  );
}

export default ElectricIconDisplay;

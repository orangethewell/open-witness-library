import { Slide } from "@mui/material";

const PageLayoutSlider = ({ children }) => {
    return (
      <Slide easing={{enter: "cubic-bezier(0.250, 0.100, 0.250, 1.000)", exit: "cubic-bezier(0.250, 0.100, 0.250, 1.000)"}} direction="up" in={true} timeout={500}>
        {children}
      </Slide>
    )
  }

export default PageLayoutSlider;
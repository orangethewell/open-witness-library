import { Fade, Slide } from "@mui/material";

const PageLayoutSlider = ({ children }) => {
    return (
      <Fade easing={{enter: "cubic-bezier(0.250, 0.100, 0.250, 1.000)", exit: "cubic-bezier(0.250, 0.100, 0.250, 1.000)"}} in={true} timeout={1000}>
        <div>
          <Slide easing={{enter: "cubic-bezier(0.250, 0.100, 0.250, 1.000)", exit: "cubic-bezier(0.250, 0.100, 0.250, 1.000)"}} direction="up" in={true} timeout={500}>
              {children}
          </Slide>
        </div>
      </Fade>

    )
  }
  

export default PageLayoutSlider;
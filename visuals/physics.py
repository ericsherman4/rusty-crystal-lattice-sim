from manim import *

# https://manimclass.com/manim-latex/
# https://www.youtube.com/watch?v=5anTYHWuMSA
# https://docs.manim.community/en/stable/guides/using_text.html


class CreateCircle(Scene):
    def construct(self):
        circle = Circle()
        circle.set_fill(PINK, opacity=0.5)
        self.play(Create(circle))


class WriteEquation(Scene):

    def match_colors(obj, colors: list):
        not_operator_indexes = [0,2,4]
        items = [obj[x] for x in not_operator_indexes]
        for item,color in zip(items, colors):
            item.set_color(color)
        


    def construct(self):

        debug = False
        debug_labels = None

        # plain english equation
        vel_plain = MathTex(
            r"{{NewVelocity}} = {{CurrentVelocity}} + {{CurrentAcceleration*TimeDifference}}",
        ).shift(UP)
        vel_plain.font_size = 35

        # Use this debugging function which will show the indexes of everything
        if debug:
            debug_labels = index_labels(vel_plain).shift(DOWN*0.5)
            self.add(debug_labels)
        
        # Set colors of the specific parts
        WriteEquation.match_colors(vel_plain, [GREEN, YELLOW, ORANGE])


        # Write
        self.play(Write(vel_plain))
        self.wait(2)
        if debug:
            self.remove(debug_labels)

        # Make a copy and display it below    
        vel_plain_clone = vel_plain.copy().shift(DOWN*1)
        self.play(TransformFromCopy(vel_plain, vel_plain_clone))
        self.wait(1)

        # Write out the velocity equation
        vel = MathTex(
            r"{{v(t + \Delta t)}} = {{v(t)}} + {{a(t)\Delta t}}",
        )

        # debug
        if debug:
            debug_labels = index_labels(vel).shift(DOWN*0.5 )
            self.add(debug_labels)

        # set colors to match that of above
        WriteEquation.match_colors(vel, [GREEN, YELLOW, ORANGE])


        # vel[4].set_color(ORANGE)
        # vel.set_color_by_tex("v", ORANGE)

        self.play(Transform(vel_plain_clone, vel), run_time =3 )
        self.wait(1)

        # Copy the velocity equation down


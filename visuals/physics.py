from manim import *


# https://manimclass.com/manim-latex/
# https://www.youtube.com/watch?v=5anTYHWuMSA
# https://docs.manim.community/en/stable/guides/using_text.html


class CreateCircle(Scene):
    def construct(self):
        circle = Circle()
        circle.set_fill(PINK, opacity=0.5)
        self.play(Create(circle))



# TODO: check out 
# https://github.com/3b1b/videos?tab=readme-ov-file#workflow
# he somehow is able to get his workflow to be more like jupyter notebook which is awesome
# he explains it a bit in the beginning of this video
# https://github.com/ManimCommunity/manim/discussions/3954 see this for a manimCE workflow convo
# sections maybe? https://docs.manim.community/en/stable/tutorials/output_and_config.html#sections 
# but idk how to use them with the sideview

class InfluenceDiagram(ThreeDScene):
    def construct(self):
        # self.next_section(skip_animations=True)
        # group = VGroup()
        # for buff in np.arange(0, 2.2, 0.45):
        #     group += Arrow(start=2*LEFT, end=2*RIGHT,color= BLUE, buff=buff, tip_shape=ArrowSquareTip) 
        # group.arrange(DOWN)
        # self.play(Create(group ))
        # self.wait(1)

        # making the component model
        group = VGroup() # does not mean vertical group
        items = ["Node Force", "Node Acceleration", "Node Velocity", "Node Position"]
        for i,item in enumerate(items):
            group += Arrow(UP*1.3,  0.0*DOWN, color =BLUE)
            group += Text(item, font_size=20)   
        
        # arrange and then animate the base component model
        self.next_section(skip_animations=False)
        group.arrange(DOWN)
        for i in range(0, len(group), 2):
            self.play(Create(group[i])) # arrow
            self.wait(0.2)
            self.play(Write(group[i+1])) # text

        # now add the topmost component which is spring stuff
        spring_stuff= Text("Spring Stuff", font_size=20)
        new_group = VGroup(spring_stuff,*group.copy())
        new_group.arrange(DOWN)
        spring_stuff_mob = new_group[0]
        self.play(group.animate.shift(group.get_top() - new_group.get_top()))
        self.wait(0.5)
        self.play(Write(spring_stuff_mob))
        self.wait(1)



        # Transform "spring stuff into spring displacement"
        spring_displacement= Text("Spring Displacement", font_size=20).move_to(spring_stuff.get_center())
        self.play(Transform(spring_stuff_mob, spring_displacement))
        self.wait(0.5)

        self.next_section()


        def zero_but_comp(comp, point3d):
            """ Zero out all the components besides the one specified """
            if comp == 0: # zero all but x
                return np.array([point3d[0], 0.0, 0.0]) 
            elif comp == 1: # zero all but y
                return np.array([0.0, point3d[1], 0.0])
            elif comp == 2: # zero all but z
                return np.array([0.0, 0.0, point3d[2]])


        # points that make up the arrow from node position to spring displacement
        points = [
            new_group[-1].get_left() + LEFT*0.3,
            new_group[-1].get_left() + LEFT*0.3 + LEFT*1.3,
            zero_but_comp(0, new_group[-1].get_left()) + LEFT*0.3 + LEFT*1.3 + zero_but_comp(1, spring_displacement.get_left()),
            spring_displacement.get_left() + LEFT*0.3,
            ]
        
        # debugging some stuff
        # for point in points:
        #     self.add(Dot().move_to(point))
        # self.add(SurroundingRectangle(new_group[-1]))
        # self.add(SurroundingRectangle(spring_stuff_mob, color=BLUE))
        # self.add(SurroundingRectangle(spring_displacement, color=BLUE))

        self.wait(1)

        # Arrow from pos to displacement
        arrow = VMobject(color=BLUE)
        arrow.add_points_as_corners(points)
        arrow_head = ArrowTriangleTip(fill_opacity=1, width=0.15, length=0.18).move_to(points[-1])
        arrow_head.rotate(PI)
        complete_arrow = VGroup(arrow, arrow_head)
        self.play(Create(complete_arrow), run_time=4)
        self.wait(1)

        # Make a group with everything
        new_group += complete_arrow

        # remove the old group because its now part of new_group
        self.remove(group)
        self.play(new_group.animate.shift(LEFT))
        self.wait(1)

        # transform all the words into functions
        # for item in new_group:
        #     if item.(isinstance)

        # self.begin_ambient_camera_rotation(rate=2, about='gamma')
        # self.wait(3)
        # self.stop_ambient_camera_rotation(about='gamma')

        # axes_3d = ThreeDAxes()
        # self.add(axes_3d)

        # Move the camera to the right
        self.set_camera_orientation(phi=0, theta=-90*DEGREES, gamma=0)
        self.move_camera( phi=self.camera.get_phi() - 20*DEGREES, run_time=2)
        self.wait()

        # Move the camera to the left
        self.move_camera(phi=self.camera.get_phi() + 40*DEGREES, run_time=2)
        self.wait()





        

        


        





class WriteEquation(Scene):

    def match_colors(obj, colors: list):
        not_operator_indexes = [0,2,4]
        items = [obj[x] for x in not_operator_indexes]
        for item,color in zip(items, colors):
            item.set_color(color)
        
    def construct(self):

        debug = False
        debug_labels = None

        # add title
        self.play(Write(Text("Euler Method").shift(UP*2)))
        self.wait(2)

        # plain english equation
        vel_plain = MathTex(
            r"{{NewVelocity}} = {{CurrentVelocity}} + {{CurrentAcceleration*TimeDifference}}",
        ).shift(UP)
        vel_plain.font_size = 38 

        # Use this debugging function which will show the indexes of everything
        if debug:
            debug_labels = index_labels(vel_plain).shift(DOWN*0.5)
            self.add(debug_labels)
        
        # Set colors of the specific parts
        colors = [ManimColor.from_hex("#45a6ad"), ManimColor.from_hex("#76b83e"), ManimColor.from_hex("#ded509") ]
        WriteEquation.match_colors(vel_plain, colors)


        # Write
        self.play(Write(vel_plain))
        self.wait(2)
        if debug:
            self.remove(debug_labels)


        # Write out the velocity equation
        vel = MathTex(
            r"{{v(t + \Delta t)}} = {{v(t)}} + {{a(t)\Delta t}}",
        )

        if debug:
            debug_labels = index_labels(vel).shift(DOWN*0.5 )
            self.add(debug_labels)

        # set colors to match that of above
        WriteEquation.match_colors(vel, colors)


        self.play(TransformFromCopy(vel_plain, vel), run_time = 2 )
        self.wait(3)

        # Copy the velocity equation down and transform into position
        pos = MathTex(
            r"{{x(t + \Delta t)}} = {{x(t)}} + {{v(t)\Delta t}}",
        )


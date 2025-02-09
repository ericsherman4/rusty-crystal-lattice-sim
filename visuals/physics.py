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

class InfluenceDiagram(Scene):
    def construct(self):
        # self.next_section(skip_animations=True)
        # group = VGroup()
        # for buff in np.arange(0, 2.2, 0.45):
        #     group += Arrow(start=2*LEFT, end=2*RIGHT,color= BLUE, buff=buff, tip_shape=ArrowSquareTip) 
        # group.arrange(DOWN)
        # self.play(Create(group ))
        # self.wait(1)

        self.next_section(skip_animations=True)


        # making the component model
        group = VGroup() # does not mean vertical group
        items = [r"Node\;Force", r"Node\;Acceleration", r"Node\;Velocity", r"Node\;Position"]
        colors = [BLUE_A, BLUE_B,BLUE_C,BLUE_D]
        for i,item in enumerate(items):
            group += Arrow(UP*1.3,  0.0*DOWN, color =colors[i])
            group += MathTex(item, font_size=35)   
        
        # arrange and then animate the base component model
        group.arrange(DOWN)
        for i in range(0, len(group), 2):
            self.play(Create(group[i])) # arrow
            self.wait(0.2)
            self.play(Write(group[i+1])) # text

        # now add the topmost component which is spring stuff
        spring_stuff= MathTex(r"{{Spring}}\;Stuff", font_size=35)
        new_group = VGroup(spring_stuff,*group.copy())
        new_group.arrange(DOWN)
        spring_stuff_mob = new_group[0]
        self.play(group.animate.shift(group.get_top() - new_group.get_top()))
        self.wait(0.5)
        self.play(Write(spring_stuff_mob))
        self.remove(spring_stuff) #remove the original since its now part of the group (i think)
        self.replace(group, new_group)
        self.wait(1)

        # Transform "spring stuff into spring displacement"
        spring_displacement_str = r"{{Spring}}\;Displacement"
        items.insert(0,spring_displacement_str)
        spring_displacement= MathTex(spring_displacement_str, font_size=35).move_to(spring_stuff.get_center())
        self.play(Transform(spring_stuff_mob, spring_displacement))
        self.wait(0.5)


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
        arrow = VMobject(color=BLUE_E)
        arrow.add_points_as_corners(points)
        arrow_head = ArrowTriangleTip(fill_opacity=1, width=0.15, length=0.18).move_to(points[-1])
        arrow_head.rotate(PI)
        complete_arrow = VGroup(arrow, arrow_head)
        self.play(Create(complete_arrow), run_time=4)
        self.wait(1)

        # Make a group with everything
        new_group += complete_arrow


        # transform all the words into functions
        # the lists are reversed because the order of the strings in the vgroup are opposite.
        # also its a lot easier to pop instead of keeping track of indexes
        items.reverse()
        corresponding_math_symbols = [r"\Delta x", r"\vec{F}", r"\vec{a}", r"\vec{v}", r"\vec{x}"]
        corresponding_math_symbols.reverse()
        animations = []
        for i in range(0,len(new_group),2):
            component_first_word = items.pop().split(r"\;")[0]
            latex = r"{{" + component_first_word + r"\;" + corresponding_math_symbols.pop() + r"}}"
            transform_into = MathTex(latex, font_size=35).move_to(new_group[i].get_center())
            animations.append(ReplacementTransform(new_group[i], transform_into))


        self.play(*animations) # play rewording everything from words into equations
        # self.play(complete_arrow.animate.shift(RIGHT * 0.7))
        # self.wait(1)
        self.play(new_group.animate.shift(LEFT), complete_arrow.animate.shift(RIGHT * 0.7 + LEFT))
        self.wait(1)

        middle_of_spring_nodeF = new_group[1].get_center() + RIGHT*3
        # REMEMBER: this is the component model, no equations yet. 
        spring_damping = MathTex(r"{{Spring}}\;Damping", font_size =35).move_to(middle_of_spring_nodeF)
        self.play(Write(spring_damping))
        self.wait(1)
        spring_velocity_eq = MathTex(r"{{Spring}}\; \vec{v}", font_size=35).move_to(middle_of_spring_nodeF)


        arrow1 = Arrow(
            new_group[0].get_right(), 
            spring_velocity_eq.get_top()+ LEFT*0.3 + UP*0.1, 
            stroke_width=4, 
            max_tip_length_to_length_ratio=0.12,
            color=BLUE_E,

        )

        arrow2 = Arrow(
            spring_velocity_eq.get_bottom() + LEFT*0.3 + DOWN*0.1,
            new_group[2].get_right(),
            stroke_width=4, 
            max_tip_length_to_length_ratio=0.12,
            color=BLUE_E,
        )


        # make a group from the spring and two other arrows and remove for now (coming back to later)
        damping_group = VGroup(arrow1, spring_velocity_eq, arrow2)
        animations = [
            Transform(spring_damping, spring_velocity_eq, replace_mobject_with_target_in_scene=True),
            Create(arrow1),
            Create(arrow2)
        ]

        for anim in animations:
            self.play(anim)

        

        self.wait(1)

        # show constants that have not been covered
        unshown_constants =  MathTex(
            r"\text{mass} (m)\\ \text{spring constant} (k)\\ \text{damping coefficient} (c)\\ \text{original spring length} (L_o)",
            font_size =35,
        ).set_color(GREEN)
        self.play(Write(unshown_constants.shift(DOWN*2).align_on_border(RIGHT)))
        self.wait(1)


        # remove the spring damping to talk about other stuff, move the network graph over.
        self.play(FadeOut(damping_group, target_position=damping_group.get_center() + RIGHT*5),
                  new_group.animate.align_on_border(LEFT))


        
        self.wait(1)

        # display hookes law
        hookes_law = MathTex(
            r"{{Node\;\vec{F}}} = -k \, \Delta x",
            font_size = 35
        ).move_to(new_group[2]).align_to(new_group[2], LEFT)
        print("HEREEEEEEEEEEE",hookes_law.get_center())
        self.play(Indicate(new_group[2], run_time=2))

        # THE BELOW WONT WORK, it keeps moving from the center which I don't really like
        # so just write over and then replace??
        # self.play(ReplacementTransform(new_group[2], hookes_law))
        self.play(Write(hookes_law))
        self.replace(new_group[2], hookes_law)

        self.next_section() #FIXME:
        self.wait(1)



        # change in spring length
        original_str = r"{{Spring\;\Delta x}}"
        eq0 = MathTex(
            original_str,
            font_size = 35,
        ).move_to(new_group[0], LEFT).align_to(new_group[0], LEFT)
        eq1 = MathTex(
            original_str + r"{{ = \text{original spring length} -}} \text{current spring length}",
            font_size = 35,
        ).move_to( new_group[0].get_center()).align_to(new_group[0], LEFT)
        eq2 = MathTex(
            original_str + r"{{ = L_o -}} \lVert\vec{x}_{node2} - \vec{x}_{node1}\rVert",
            font_size = 35,
        ).move_to( new_group[0].get_center()).align_to(new_group[0], LEFT)

        # this to prevent some weird bs where it thought the strings were different
        # so it would apply a transform information of the same object into itself
        self.remove(new_group[0])
        new_group[0] = eq0 
        self.play(Indicate(new_group[0], run_time=2))
        self.play(Write(eq1))
        self.wait(1)
        self.play(ReplacementTransform(eq1, eq2))
        new_group[0] = eq2
        self.wait(1)
        
        # self.add(index_labels(new_group).shift(RIGHT))
        # self.add(index_labels(new_group[0][2]).shift(DOWN))
        rec1 = SurroundingRectangle(new_group[0][2][1:8])
        rec2 = SurroundingRectangle(new_group[0][2][9:16])
        # self.play(Create(rec1), Create(rec2))
        # remaking the text because surround isnt working when its in the group
        rec3 = SurroundingRectangle(MathTex(r"Node \; x", font_size = 35).move_to(new_group[8]))
        self.play(Create(rec3))
        self.play(TransformFromCopy(rec3, rec1), Transform(rec3, rec2))
        self.wait(1)
        self.play(FadeOut(rec3, rec2, rec1))
        self.wait(1)



        fma = MathTex(r"\vec{F}",  r"=",  r"m", r"\vec{a}")
        self.play(Write(fma), Flash(fma, flash_radius=fma.get_right()[0]+0.2, run_time = 2, line_length=0.6))
        afm = MathTex(r"\vec{a}", r"=", r"{" ,  r"\vec{F}", r"\over", r"m", r"}")
        self.wait(0.5)
        self.play(TransformMatchingTex(fma, afm))
        self.wait(1)


        return


        original_str = r"{{Spring \; \Delta \vec{x}}}"
        eq0 = MathTex(
            original_str,
            font_size = 35,
        ).move_to(new_group[0], LEFT).align_to(new_group[0], LEFT)
        eq1 = MathTex(
            original_str + r"{{ = \text{original spring length} -}} \text{current spring length}",
            font_size = 35,
        ).move_to( new_group[0].get_center()).align_to(new_group[0], LEFT)
        eq2 = MathTex(
            original_str + r"{{ = \text{original spring length} -}} (node_2 - node_1)",
            font_size = 35,
        ).move_to( new_group[0].get_center()).align_to(new_group[0], LEFT)


        # this to prevent some weird bs where it thought the strings were different
        # so it would apply a trasnform information of the same object into itself
        self.remove(new_group[0])
        new_group[0] = eq0 
        # self.play(Transform(new_group[0], eq0, replace_mobject_with_target_in_scene=True))
        # self.wait(1)
        self.play(Transform(new_group[0], eq1, replace_mobject_with_target_in_scene=True))
        self.wait(1)


        self.play(Transform(eq1, eq2, replace_mobject_with_target_in_scene=True))



        return 

        test1 = MathTex(r"{{Spring \; \Delta \vec{x}}}",
                        font_size=35)

        test2 = MathTex(r"{{Spring \; \Delta \vec{x}}} = x + 3 -4 * a + \text{current spring length}",
                        font_size= 35).align_to(test1, LEFT)
        
        test3 = MathTex(r"{{Spring \; \Delta \vec{x}}} = x + 3 -4 * a + (node_2 - node_1)",
                        font_size= 35).align_to(test1, LEFT)
        
        
        self.play(Write(test1))
        self.wait(1)
        self.play(Transform(test1,test2, replace_mobject_with_target_in_scene=True))
        self.wait(1)
        self.play(TransformMatchingTex(test2, test3,replace_mobject_with_target_in_scene=True))
        self.wait(1)



















        

        


        





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


package com.mygdx.game.simulation.ui.windows;

import com.badlogic.gdx.scenes.scene2d.Actor;
import com.badlogic.gdx.scenes.scene2d.utils.ChangeListener;
import com.kotcrab.vis.ui.util.form.SimpleFormValidator;
import com.kotcrab.vis.ui.widget.VisLabel;
import com.kotcrab.vis.ui.widget.VisTextButton;
import com.kotcrab.vis.ui.widget.VisValidatableTextField;
import com.kotcrab.vis.ui.widget.VisWindow;
import com.mygdx.game.simulation.SimulationScreen;

/**
 * Created by Fran on 4/21/2018.
 */
public class SimSpeedWindow extends VisWindow{
    final VisValidatableTextField field;

    public SimSpeedWindow() {
        super("Simulation Speed");
        field = new VisValidatableTextField(SimulationScreen.simSpeed+"");
        addWidgets();
        pack();
    }

    private void addWidgets(){
        VisLabel label = new VisLabel("Simulation speed: ");

        VisTextButton button = new VisTextButton("Change");


        button.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                if(!field.isDisabled()){
                    SimulationScreen.simSpeed = Float.parseFloat(field.getText());
                }
            }
        });

        initialize(field,button).floatNumber(field,"");

        add(label);
        add(field);
        add(button).width(80f);
    }

    protected SimpleFormValidator initialize(final VisValidatableTextField textField, final VisTextButton textButton){
        SimpleFormValidator validator = new SimpleFormValidator(textButton);
        textField.setDisabled(true);
        validator.notEmpty(textField,"");
        textButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                if(textField.isDisabled()){
                    textButton.setText("Set");
                } else {
                    textButton.setText("Change");
                }
                textField.setDisabled(!textField.isDisabled());
            }
        });
        return validator;
    }

    @Override
    public void act(float delta) {
        super.act(delta);
        if(field.isDisabled()){
            field.setText(SimulationScreen.simSpeed+"");
        }
    }
}

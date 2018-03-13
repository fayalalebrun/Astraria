package com.mygdx.game.simulation;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.utils.BufferUtils;
import com.mygdx.game.BaseScreen;
import com.mygdx.game.Boot;

import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.FloatBuffer;
import java.nio.IntBuffer;

/**
 * Created by fraayala19 on 12/12/17.
 */
public class SimulationScreen extends BaseScreen {

    int VAO;
    int shaderProgram;

    float vertices[] = {
            -0.5f, -0.5f, 0.0f,
            0.5f, -0.5f, 0.0f,
            0.0f,  0.5f, 0.0f
    };

    String vert = "#version 330 core\n" +
            "layout (location = 0) in vec3 aPos;\n" +
            "\n" +
            "void main()\n" +
            "{\n" +
            "    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);\n" +
            "}";

    String frag = "#version 330 core\n" +
            "out vec4 FragColor;\n" +
            "\n" +
            "void main()\n" +
            "{\n" +
            "    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);\n" +
            "} ";


    public SimulationScreen(Boot boot) {
        super(boot);


        int vertexShader = Gdx.gl.glCreateShader(Gdx.gl.GL_VERTEX_SHADER);
        Gdx.gl.glShaderSource(vertexShader, vert);
        Gdx.gl.glCompileShader(vertexShader);

        ByteBuffer temp = ByteBuffer.allocateDirect(4);
        temp.order(ByteOrder.nativeOrder());
        IntBuffer success = temp.asIntBuffer();
        Gdx.gl.glGetShaderiv(vertexShader,Gdx.gl.GL_COMPILE_STATUS,success);
        if(success.get(0)==0){
            System.out.println(Gdx.gl.glGetShaderInfoLog(vertexShader));
        }

        int fragmentShader = Gdx.gl.glCreateShader(Gdx.gl.GL_FRAGMENT_SHADER);
        Gdx.gl.glShaderSource(fragmentShader, frag);
        Gdx.gl.glCompileShader(fragmentShader);


        Gdx.gl.glGetShaderiv(fragmentShader,Gdx.gl.GL_COMPILE_STATUS,success);

        if(success.get(0)==0){
            System.out.println(Gdx.gl.glGetShaderInfoLog(fragmentShader));
        }

        shaderProgram = Gdx.gl.glCreateProgram();

        Gdx.gl.glAttachShader(shaderProgram, vertexShader);
        Gdx.gl.glAttachShader(shaderProgram, fragmentShader);
        Gdx.gl.glLinkProgram(shaderProgram);

        Gdx.gl.glGetProgramiv(shaderProgram, Gdx.gl.GL_LINK_STATUS, success);

        if(success.get(0)==0){
            System.out.println(Gdx.gl.glGetShaderInfoLog(shaderProgram));
        }

        Gdx.gl.glUseProgram(shaderProgram);

        Gdx.gl.glDeleteShader(vertexShader);
        Gdx.gl.glDeleteShader(fragmentShader);

        int[] tempv = new int[1];

        Gdx.gl30.glGenVertexArrays(1, tempv, 0);

        VAO = tempv[0];
        int VBO = Gdx.gl.glGenBuffer();

        Gdx.gl30.glBindVertexArray(VAO);

        Gdx.gl.glBindBuffer(Gdx.gl.GL_ARRAY_BUFFER, VBO);
        FloatBuffer vertexData = BufferUtils.newFloatBuffer(vertices.length*3);
        vertexData.put(vertices);
        vertexData.flip();
        Gdx.gl.glBufferData(Gdx.gl.GL_ARRAY_BUFFER, vertices.length*4,vertexData, Gdx.gl.GL_STATIC_DRAW);

        Gdx.gl.glVertexAttribPointer(0, 3, Gdx.gl.GL_FLOAT, false,12, 0);
        Gdx.gl.glEnableVertexAttribArray(0);

    }

    @Override
    public void show() {


    }


    @Override
    public void render(float delta) {

        Gdx.gl.glClearColor(0.2f,0.0f,0.3f,1.0f);
        Gdx.gl.glClear(Gdx.gl.GL_COLOR_BUFFER_BIT);


        Gdx.gl.glUseProgram(shaderProgram);
        Gdx.gl30.glBindVertexArray(VAO);
        Gdx.gl.glDrawArrays(Gdx.gl.GL_TRIANGLES, 0, 3);

    }



    @Override
    public void resume() {

    }

    @Override
    public void hide() {

    }

    @Override
    public void pause() {

    }

    @Override
    public void resize(int width, int height) {
    }


    @Override
    public void dispose() {
    }

}

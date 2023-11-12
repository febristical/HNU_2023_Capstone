#set text(font: "KoPubDotum_Pro", size: 24pt, weight: "medium")
#set page(width: 960pt, height: 540pt, margin: 7.5%)

#let bigtitle(alignment: center, body) = align(
    alignment,
    text(font: "KoPubDotum_Pro", weight: "black", size: 54pt, body)
)

#let verifier(school, author) = align(
    bottom + right,
    stack(
        dir:ttb,
        spacing: 10pt,
        text(size: 20pt, school),
        text(size: 20pt, author)
    )
)

#let title(alignment: center, body) = align(
    alignment,
    text(font: "KoPubDotum_Pro", weight: "black", size: 36pt, body)
)

#let contentsbox(
    alignment: horizon, nameweight: "black", name, primary, secondary
) = align(
    alignment,
    rect(
        width: 100%,
        fill: rgb("F0F0F0"),
        radius: 24pt,
        outset: 10pt,
        stack(
            dir:ttb,
            spacing: 24pt,
            text(weight: nameweight, name) + h(5pt) + primary,
            secondary
        )
    )
)

#bigtitle(alignment: horizon + center, "Ray Marching 을 이용한 3D 렌더링")

#verifier("한남대학교 수학과", "20172581 김남훈")

#pagebreak()

#contentsbox("3D 렌더링이란?", "3차원 모델로부터 사실적인 2D 이미지를 만들어내는 절차",
figure(
    image(
        "Raytraced_ball.png",
        height: 75%
    ),
    caption: stack(dir:ttb, spacing: 5pt,
        text(size: 20pt, "재귀적 레이 트레이싱 기법으로 렌더링된 구(Ball)"),
        text(size: 12pt, "출처: https://commons.wikimedia.org/wiki/File:Recursive_raytrace_of_a_sphere.png#/media/File:Recursive_raytrace_of_a_sphere.png")
    ),
    supplement: none
)
)

#pagebreak()

#contentsbox(
    "렌더링은 어떻게 하는가?",
    "다양한 렌더링 기법 존재",
    "
1. 패스 트레이싱(Path Tracing)
2. 복셀 콘 트레이싱(Voxel Cone Tracing)
3. 레이 마칭(Ray Marching)
4. ...
    "
)

#pagebreak()

#contentsbox(
    "부호화 거리 함수(Signed Distance Field)", [점 $P$ 가 도형 $S$ 의 내부에 있을 때는 음수, 외부에 있을 때는 양수로 $P$ 와 $S$ 의 거리를 나타내는 함수],
    [
        $P$ 를 점, $S$ 를 도형, $diff S$ 를 $S$ 의 표면이라고 하자. 그리고 $d(P, diff S)$ 를 $P$ 와 $diff S$ 사이의 거리라고 하면, 부호화 거리 함수 $"SDF"$ 는

        $
            "SDF"(P, S) = cases(
                d(P, diff S) &"if"& P in.not S \
                -d(P, diff S) &"if"& P in S
            )
        $

        로 정의한다.
    ]
)

#pagebreak()

#contentsbox(
    "레이 마칭",
    "레이 마칭은 부호화 거리 함수를 이용하여 주어진 방향에 물체가 존재하는지 판별하는 알고리즘이다. 다음의 과정을 따라 진행된다.",
    [
        먼저 $C$ 를 카메라(관찰자)의 위치, $S$ 를 그리고자 하는 도형, $v$ 를 단위 벡터, $epsilon$ 을 허용 오차, $D$ 를 최대 전진 거리라 하자.

        + $d_0 = 0, P_0 = C$ 로 놓는다. 이 때, $"SDF(P_0, S) > 0"$ 이라 가정한다.
        + $i$ 를 $0$ 에서부터 $1$ 씩 증가시키며, $d_i < epsilon$ 또는 $d_0 + dots + d_i > D$ 가 될 때까지 다음을 반복한다.
            + $d_(i + 1) = "SDF"(P_i, S)$
            + $P_(i + 1) = P_i + d_(i + 1) v$
        + $d_i < epsilon$ 이면 $v$ 방향에 $S$ 가 존재하며, $d_0 + dots + d_i > D$ 이면 $v$ 방향에 $S$ 가 존재하지 않는다.
    ]
)

#pagebreak()

#figure(
    box(
        stroke: luma(0),
        image(
        "march.png"
        )
    ),
    caption: text(size: 14pt, "레이 마칭으로 왼쪽 위의 빨간 점을 오른쪽 아래로 전진시키는 과정" ),
    supplement: none
)
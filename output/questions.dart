Map<String, dynamic> initialData = Map<String, dynamic>();

this.initialData = const {
        'Header': 'Match Scouting',
        'Team Number': 0,
'lanugage':'',
'wheel type':'',
'linux distro':'',

//new question
'nice level':0,
'rust level':0,
'coolness':0,

//new question
'Question':['answer2','answer1',],
'Question2':['answer2','answer1',],
};
List<Question>? matchFormQuestions;
matchFormQuestions = [ShortAnswer(
        'Team Number',
        TextInputType.number,
        initialValue: widget.initialData['Team Number'],
      ),
ShortAnswer(
'coolness',
TextInputType.number,
initialValue: widget.initialData['coolness'],
),
ShortAnswer(
'rust level',
TextInputType.number,
initialValue: widget.initialData['rust level'],
),
ShortAnswer(
'nice level',
TextInputType.number,
initialValue: widget.initialData['nice level'],
),
ShortAnswer(
'linux distro',
TextInputType.text,
initialValue: widget.initialData['linux distro'],
),
ShortAnswer(
'wheel type',
TextInputType.text,
initialValue: widget.initialData['wheel type'],
),
ShortAnswer(
'lanugage',
TextInputType.text,
initialValue: widget.initialData['lanugage'],
),
DropDownQuestion(
'Question',
['answer2','answer1',],
,answer: widget.initialData['Question']
),
DropDownQuestion(
'Question2',
['answer2','answer1',],
,answer: widget.initialData['Question2']
),
];
